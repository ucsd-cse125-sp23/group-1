use rapier3d::prelude::*;
use nalgebra::{UnitQuaternion,Isometry3,Translation3,Vector3};
use slotmap::{SlotMap, SecondaryMap, DefaultKey, Key, KeyData};
use std::{str};
use std::io::{Read, Write, self};
use std::net::{TcpListener};

use shared::shared_components::*;
use crate::server_components::*;

type Entity = DefaultKey;

pub struct ECS {
    pub name_components: SlotMap<Entity, String>,
    
    // shared components
    pub player_input_components: SecondaryMap<Entity, PlayerInputComponent>,
    pub position_components: SecondaryMap<Entity, PositionComponent>,
    pub player_weapon_components: SecondaryMap<Entity, PlayerWeaponComponent>,
    pub model_components: SecondaryMap<Entity, ModelComponent>,
    
    // server components
    pub physics_components: SecondaryMap<Entity, PhysicsComponent>,
    pub network_components: SecondaryMap<Entity, NetworkComponent>,
    pub health_components: SecondaryMap<Entity, HealthComponent>,
    pub player_camera_components: SecondaryMap<Entity, PlayerCameraComponent>,

    pub players: Vec<Entity>,
    pub dynamics: Vec<Entity>,
    pub renderables: Vec<Entity>,

    pub temp_entity: Entity,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            name_components: SlotMap::new(),
            player_input_components: SecondaryMap::new(),
            position_components: SecondaryMap::new(),
            player_weapon_components: SecondaryMap::new(),
            model_components: SecondaryMap::new(),
            physics_components: SecondaryMap::new(),
            network_components: SecondaryMap::new(),
            health_components: SecondaryMap::new(),
            player_camera_components: SecondaryMap::new(),
            players: vec![],
            dynamics: vec![],
            renderables: vec![],
            temp_entity: DefaultKey::default(),
        }
    }

    /**
     * Listens to accept client connection, adds player (name, rigid body set, collider set),
     * updates ECS network components
     *
     * @param listener: provides TCP server socket support
     * @param rigid_body_set
     * @param collider_set
     */
    pub fn connect_client(&mut self, listener: &TcpListener, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
        match listener.accept() {
            Ok((stream, addr)) => {
                let mut curr_stream = stream;
                println!("Client connected: {addr:?}, client id: {}", self.players.len());
                // send client id -- supports no more than 255 clients
                let client_id = self.players.len() as u8;
                // if this error happens something is very wrong with the connection
                curr_stream.write(&[client_id]).unwrap();
                curr_stream.set_nonblocking(true).expect("Failed to set stream as nonblocking");
                let name = "dummy".to_string();     // TODO: get name from client
                let player = self.new_player(name.clone(),rigid_body_set,collider_set);
                self.network_components.insert(player, NetworkComponent { stream:curr_stream });
                self.health_components.insert(player, HealthComponent { alive:true, health:1 });
                println!("Name: {}", name);
            },
            Err(e) => {
                eprintln!("Failed to connect to client: {e:?}");
            },
        }
    }

    /**
     * For all ECS players, uses the respective stream to read + deserialize messages, updates
     * ECS player input components for each player
     */
    pub fn receive_inputs(&mut self) {
        for &player in &self.players {
            // do not receive inputs from dead players
            let health = & self.health_components[player].alive;
            if !health {
                continue;
            }

            let mut input_temp = PlayerInputComponent::default();
            let mut stream = & self.network_components[player].stream;
            // need a protocol, get number of bytes in message then read_exact

            // read messages from client with header length
            // 4 byte size field
            loop {
                let mut size_buf = [0 as u8; 4];
                let size:u32;
                match stream.peek(&mut size_buf) {
                    Ok(4) => {
                        // it's tradition, dammit!
                        size = u32::from_be_bytes(size_buf);
                    },
                    Ok(_) => {
                        // is this necessary? nonblocking might already handle this. worth testing
                        break;
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to read message size for client {}: {}",self.name_components[player],e);
                        // TODO: handle lost client
                        panic!("Lost client connection");
                    }
                }
                let s_size = size.try_into().unwrap();
                let mut read_buf = vec![0 as u8; s_size];
                // might be able to just read instead of peeking, requires testing
                match stream.peek(&mut read_buf) {
                    Ok(bytes_read) if bytes_read == s_size => {
                        // if this throws an error we deserve to crash tbh
                        stream.read_exact(&mut read_buf).expect("read_exact did not read the same amount of bytes as peek");
                        let message : &str = str::from_utf8(&read_buf[4..]).expect("Error converting buffer to string");
                        let value : PlayerInputComponent = serde_json::from_str(message).expect("Error converting string to PlayerInputComponent");
                        ECS::combine_input(&mut input_temp, value);
                    },
                    Ok(_) => {
                        break;
                    },
                    Err(e) => {
                        eprintln!("Failed to read message for client {}: {}",self.name_components[player],e);
                    },
                }
            }
            // once all inputs have been aggregated for this player
            let camera = &mut self.player_camera_components[player];
            camera.camera_front = vector![input_temp.camera_front_x, input_temp.camera_front_y, input_temp.camera_front_z].normalize();
            camera.camera_right = camera.camera_front.cross(&Vector3::y()).normalize();
            camera.camera_up = camera.camera_right.cross(&camera.camera_front).normalize();
            self.player_input_components[player] = input_temp;
        }
    }

    /**
     * Given a new player input component, updates the current component
     *
     * @param curr: player input component to be updated
     * @param value: player input component to be saved
     */ // may God have mercy on our souls
    pub fn combine_input(curr: &mut PlayerInputComponent, value: PlayerInputComponent) {
        curr.lmb_clicked |=  value.lmb_clicked;
        curr.rmb_clicked |= value.rmb_clicked;
        curr.w_pressed |= value.w_pressed;
        curr.a_pressed |= value.a_pressed;
        curr.s_pressed |= value.s_pressed;
        curr.d_pressed |= value.d_pressed;
        curr.shift_pressed |= value.shift_pressed;
        curr.ctrl_pressed |= value.ctrl_pressed;
        curr.r_pressed |= value.r_pressed;
        curr.camera_front_x = value.camera_front_x;
        curr.camera_front_y = value.camera_front_y;
        curr.camera_front_z = value.camera_front_z;
    }

    /**
     * Using TCP streams associated with each player, send updated + serialized client ECS
     */
    pub fn update_clients(&mut self) {
        let client_ecs = self.client_ecs();
        let j = serde_json::to_string(&client_ecs).expect("Client ECS serialization error");
        let size = j.len() as u32 + 4;
        for &player in &self.players {
            let message = [u32::to_be_bytes(size).to_vec(), j.clone().into_bytes()].concat();
            match self.network_components[player].stream.write(&message) {
                Ok(_) => (),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
                Err(e) => panic!("Error updating client \"{}\": {:?}", self.name_components[player], e),
            }
        }
    }

    /**
     * Creates client ECS, containing names + position components of all objects and players data
     */
    pub fn client_ecs(&self) -> ClientECS {
        ClientECS {
            name_components: self.name_components.clone(),
            position_components: self.position_components.clone(),
            model_components: self.model_components.clone(),
            health_components: self.health_components.clone(),
            players: self.players.clone(),
            renderables: self.renderables.clone(),
        }
    }

    /**
     * Creates a new player
     *
     * @param name
     * @param rigid_body_set
     * @param collider_set
     * 
     * @return an Entity that represents the player
     */
    pub fn new_player(&mut self, name: String, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Entity {
        let player = self.name_components.insert(name);
        self.players.push(player);
        self.dynamics.push(player);
        self.renderables.push(player);
        self.model_components.insert(player, ModelComponent { modelname: "cube".to_string() });
        self.player_input_components.insert(player, PlayerInputComponent::default());
        self.position_components.insert(player, PositionComponent::default());
        self.player_weapon_components.insert(player, PlayerWeaponComponent{cooldown: 0, ammo: 6, reloading: false});
        self.player_camera_components.insert(player, PlayerCameraComponent{camera_front: vector![0.0, 0.0, 0.0],camera_up: vector![0.0, 0.0, 0.0],camera_right: vector![0.0, 0.0, 0.0]});
        let rigid_body = RigidBodyBuilder::dynamic().translation(vector![0.0, 0.0, 2.0]).lock_rotations().can_sleep(false).build();
        let handle = rigid_body_set.insert(rigid_body);
        let collider = ColliderBuilder::capsule_y(1.0, 0.5).user_data(player.data().as_ffi() as u128).build();
        let collider_handle = collider_set.insert_with_parent(collider, handle, rigid_body_set);
        self.physics_components.insert(player,PhysicsComponent{handle, collider_handle});
        player
    }

    pub fn spawn_prop(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, 
        name: String, modelname: String, pos_x: f32, pos_y: f32, pos_z: f32, rot_x: f32, rot_y: f32, rot_z: f32, 
        dynamic: bool, shape: SharedShape, density: f32, restitution: f32) {
            let entity = self.name_components.insert(name);
            self.renderables.push(entity);
            let rot = UnitQuaternion::from_euler_angles(rot_x,rot_y,rot_z);
            self.position_components.insert(entity, PositionComponent { x: (pos_x), y: (pos_y), z: (pos_z), qx: (rot.i), qy: (rot.j), qz: (rot.k), qw: (rot.w) });
            self.model_components.insert(entity,ModelComponent { modelname });
            let rigid_body: RigidBody;
            if dynamic {
                self.dynamics.push(entity);
                rigid_body = RigidBodyBuilder::dynamic().position(Isometry3::from_parts(Translation3::new(pos_x, pos_y, pos_z),rot)).can_sleep(false).build();
            } else {
                rigid_body = RigidBodyBuilder::fixed().position(Isometry3::from_parts(Translation3::new(pos_x, pos_y, pos_z),rot)).build();
            }
            let handle = rigid_body_set.insert(rigid_body);
            let collider = ColliderBuilder::new(shape).density(density).restitution(restitution).user_data(entity.data().as_ffi() as u128).build();
            let collider_handle = collider_set.insert_with_parent(collider, handle, rigid_body_set);
            self.physics_components.insert(entity, PhysicsComponent { handle, collider_handle });

    }

    /**
     * Updates position components of all objects in the game
     *
     * @param rigid_body_set
     */
    pub fn update_positions(&mut self, rigid_body_set: &mut RigidBodySet) {
        for &dynamic in &self.dynamics {
            let rigid_body = rigid_body_set.get(self.physics_components[dynamic].handle).unwrap();
            let mut position = &mut self.position_components[dynamic];
            position.x = rigid_body.translation().x;
            position.y = rigid_body.translation().y;
            position.z = rigid_body.translation().z;
            position.qx = rigid_body.rotation().i;
            position.qy = rigid_body.rotation().j;
            position.qz = rigid_body.rotation().k;
            position.qw = rigid_body.rotation().w;
        }
    }

    /**
     * TODO: add description
     *
     * @param rigid_body_set
     */
    pub fn player_fire(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, query_pipeline: & QueryPipeline) {
        for &player in &self.players {
            let mut weapon = &mut self.player_weapon_components[player];
            let input = &self.player_input_components[player];
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
                if weapon.reloading && weapon.cooldown == 0 {
                    weapon.ammo = 6;
                    weapon.reloading = false;
                    println!("ammo: {}",weapon.ammo);
                }
            }
            if input.lmb_clicked && weapon.cooldown == 0 && weapon.ammo > 0 {
                println!("firing!");
                let fire_vec = &self.player_camera_components[player].camera_front;
                let impulse = 10.0 * fire_vec;
                let position = &self.position_components[player];

                let ray = Ray::new(point![position.x, position.y, position.z], *fire_vec);
                let max_toi = 1000.0; //depends on size of map
                let solid = true;
                let filter = QueryFilter::new().exclude_rigid_body(self.physics_components[player].handle);
                match query_pipeline.cast_ray(rigid_body_set, collider_set, &ray, max_toi, solid, filter) {
                    Some((target_collider_handle, toi)) => {
                        let target_collider = collider_set.get_mut(target_collider_handle).unwrap();
                        let target = DefaultKey::from(KeyData::from_ffi(target_collider.user_data as u64));
                        
                        let target_name = & self.name_components[target];
                        println!("Hit target {}",target_name);

                        // if target is a player, update its health component
                        if self.players.contains(&target) && self.health_components[target].alive {
                            self.health_components[target].health -= 1;
                            
                            if self.health_components[target].health == 0 {
                                // handle player death
                                self.health_components[target].alive = false;
                                self.player_input_components[target] = PlayerInputComponent::default();
                            }
                        }
                        
                        let hit_point = ray.point_at(toi);
                        let target_body = rigid_body_set.get_mut(self.physics_components[target].handle).unwrap();
                        target_body.apply_impulse_at_point(impulse, hit_point, true);

                    },
                    None => {
                        println!("Miss");
                    },
                }

                let rigid_body = rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
                rigid_body.apply_impulse(-impulse, true);
                // weapon cooldown is measured in ticks
                weapon.cooldown = 30;
                weapon.ammo -= 1;
                println!("ammo: {}",weapon.ammo);
            } else if (input.lmb_clicked || (input.r_pressed && weapon.ammo < 6)) && weapon.cooldown == 0 {
                println!("reloading...");
                weapon.cooldown = 180;
                weapon.reloading = true;
            }
        }
    }

    pub fn player_move(&mut self, rigid_body_set: &mut RigidBodySet) {
        for &player in &self.players {
            let input = &self.player_input_components[player];
            let camera = &self.player_camera_components[player];
            let impulse = 0.05;
            let rigid_body = rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
            if input.w_pressed && !input.s_pressed {
                rigid_body.apply_impulse(impulse * camera.camera_front, true);
            }
            if input.s_pressed && !input.w_pressed {
                rigid_body.apply_impulse(-impulse * camera.camera_front, true);
            }
            if input.a_pressed && !input.d_pressed {
                rigid_body.apply_impulse(-impulse * camera.camera_right, true);
            }
            if input.d_pressed && !input.a_pressed {
                rigid_body.apply_impulse(impulse * camera.camera_right, true);
            }
            if input.shift_pressed && !input.ctrl_pressed {
                rigid_body.apply_impulse(impulse * camera.camera_up, true);
            }
            if input.ctrl_pressed && !input.shift_pressed {
                rigid_body.apply_impulse(-impulse * camera.camera_up, true);
            }
        }
    }
}