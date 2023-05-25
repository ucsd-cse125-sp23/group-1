use rapier3d::prelude::*;
use nalgebra::{UnitQuaternion, Isometry3, Translation3, Quaternion, distance};
use slotmap::{SlotMap, SecondaryMap, DefaultKey, Key, KeyData};
use std::str;
use std::io::{Read, Write, self};
use std::net::TcpListener;
use rand::{thread_rng,seq::IteratorRandom};

use shared::*;
use shared::shared_components::*;
use crate::{server_components::*, init_world::*};


type Entity = DefaultKey;

pub struct ECS {
    pub name_components: SlotMap<Entity, String>,
    
    // shared components
    pub player_input_components: SecondaryMap<Entity, PlayerInputComponent>,
    pub position_components: SecondaryMap<Entity, PositionComponent>,
    pub player_weapon_components: SecondaryMap<Entity, PlayerWeaponComponent>,
    pub player_lasso_components: SecondaryMap<Entity, PlayerLassoComponent>,
    pub model_components: SecondaryMap<Entity, ModelComponent>,
    pub player_health_components: SecondaryMap<Entity, PlayerHealthComponent>,
    
    // server components
    pub physics_components: SecondaryMap<Entity, PhysicsComponent>,
    pub network_components: SecondaryMap<Entity, NetworkComponent>,
    pub player_camera_components: SecondaryMap<Entity, PlayerCameraComponent>,
    pub player_lasso_phys_components: SecondaryMap<Entity, PlayerLassoPhysComponent>,

    // physics objects
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,

    pub ready_players: SecondaryMap<Entity, bool>,

    pub ids: Vec<Entity>,
    pub players: Vec<Entity>,
    pub dynamics: Vec<Entity>,
    pub renderables: Vec<Entity>,

    pub spawnpoints: Vec<Isometry3<f32>>,
    pub active_players: u8,
    pub game_ended: bool,
}

impl ECS {
    /**
     * Initialize an ECS
     */
    pub fn new() -> ECS {
        ECS {
            name_components: SlotMap::new(),

            player_input_components: SecondaryMap::new(),
            position_components: SecondaryMap::new(),
            player_weapon_components: SecondaryMap::new(),
            player_lasso_components: SecondaryMap::new(),
            model_components: SecondaryMap::new(),
            player_health_components: SecondaryMap::new(),

            physics_components: SecondaryMap::new(),
            network_components: SecondaryMap::new(),
            player_camera_components: SecondaryMap::new(),
            player_lasso_phys_components: SecondaryMap::new(),

            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(), 
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),

            ready_players: SecondaryMap::new(),

            ids: vec![],
            players: vec![],
            dynamics: vec![],
            renderables: vec![],

            spawnpoints: vec![],
            active_players: 0,
            game_ended: false,
        }
    }

    /**
     * Clear ECS of everything but players, reset all the components/fields for a game restart
     */
    pub fn reset(&mut self) {
        // remove disconnected players
        let mut disconnected_players: Vec<Entity> = vec![];
        for &player in &self.players {
            if !self.network_components[player].connected {
                disconnected_players.push(player);
            }
        }
        for player in disconnected_players{
            self.remove_player(player);
        }

        // reset physics objects
        self.rigid_body_set = RigidBodySet::new();
        self.collider_set = ColliderSet::new();
        self.physics_pipeline = PhysicsPipeline::new();
        self.island_manager = IslandManager::new();
        self.broad_phase = BroadPhase::new();
        self.narrow_phase = NarrowPhase::new();
        self.impulse_joint_set = ImpulseJointSet::new();
        self.multibody_joint_set = MultibodyJointSet::new();
        self.ccd_solver = CCDSolver::new();
        self.query_pipeline = QueryPipeline::new();

        // clear ECS of everything but players
        self.name_components.retain(|key, _| self.players.contains(&key));
        self.player_input_components.retain(|key, _| self.players.contains(&key));
        self.position_components.retain(|key, _| self.players.contains(&key));
        self.player_weapon_components.retain(|key, _| self.players.contains(&key));
        self.model_components.retain(|key, _| self.players.contains(&key));
        self.player_health_components.retain(|key, _| self.players.contains(&key));
        self.physics_components.retain(|key, _| self.players.contains(&key));
        self.network_components.retain(|key, _| self.players.contains(&key));
        self.player_camera_components.retain(|key, _| self.players.contains(&key));
        self.player_lasso_components.clear();
        self.player_lasso_phys_components.clear();
        self.ready_players.clear();
        self.dynamics.clear();
        self.renderables.clear();

        init_world(self);
        init_player_spawns(&mut self.spawnpoints);

        for &player in &self.players {
            if !self.network_components[player].connected {
                panic!("a disconnected player found");
            }

            self.player_input_components[player] = PlayerInputComponent::default();
            self.player_weapon_components[player] = PlayerWeaponComponent::default();
            self.player_camera_components[player] = PlayerCameraComponent::default();
            self.player_health_components[player] = PlayerHealthComponent::default();

            if self.spawnpoints.is_empty() {
                eprintln!("Ran out of player spawnpoints, reusing");
                init_player_spawns(&mut self.spawnpoints);
            }
            let player_pos = self.spawnpoints.swap_remove((0..self.spawnpoints.len()).choose(&mut thread_rng()).unwrap());
            self.position_components[player] = PositionComponent{
                x: player_pos.translation.x,
                y: player_pos.translation.y,
                z: player_pos.translation.z,
                qx: player_pos.rotation.i,
                qy: player_pos.rotation.j,
                qz: player_pos.rotation.k,
                qw: player_pos.rotation.w,
            };
            let rigid_body = RigidBodyBuilder::dynamic().position(player_pos).lock_rotations().can_sleep(false).build();
            let handle = self.rigid_body_set.insert(rigid_body);
            let collider = ColliderBuilder::capsule_y(0.5, 0.4).user_data(player.data().as_ffi() as u128).build();
            let collider_handle = self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);
            self.physics_components[player] = PhysicsComponent{handle, collider_handle};
            self.dynamics.push(player);
            self.renderables.push(player);
        }

        self.active_players = self.players.len() as u8;
        self.game_ended = false;
    }

    /**
     * Listens to accept client connection, adds player (name, rigid body set, collider set),
     * updates ECS network components
     *
     * @param   listener: provides TCP server socket support
     */
    pub fn connect_client(&mut self, listener: &TcpListener) {
        match listener.accept() {
            Ok((stream, addr)) => {
                let mut curr_stream = stream;
                // send client id -- supports no more than 255 clients
                let client_id = self.ids.len() as u8;
                // will most likely trigger if client exits after requesting to connect
                if curr_stream.write(&[client_id]).is_err() {
                    eprintln!("Skipping invalid client connection");
                    return; // skip adding this client
                }
                println!("Client connected: {addr:?}, client id: {}", self.ids.len());
                curr_stream.set_nonblocking(true).expect("Failed to set stream as nonblocking");
                let name = "dummy".to_string();     // TODO: get name from client
                let player = self.new_player(name.clone());
                self.network_components.insert(player, NetworkComponent{connected: true, stream: curr_stream});
                self.player_health_components.insert(player, PlayerHealthComponent::default());
                self.active_players += 1;
                self.send_ready_message(false);
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
        let mut disconnected_players: Vec<Entity> = vec![];

        for &player in &self.players {
            let mut connected = true;

            // do not receive inputs from dead players
            let health = & self.player_health_components[player].alive;
            if !health {
                continue;
            }

            let mut input_temp = PlayerInputComponent::default();
            input_temp.camera_qw = self.player_input_components[player].camera_qw;
            input_temp.camera_qx = self.player_input_components[player].camera_qx;
            input_temp.camera_qy = self.player_input_components[player].camera_qy;
            input_temp.camera_qz = self.player_input_components[player].camera_qz;

            let mut stream = & self.network_components[player].stream;

            let mut received_input = false;

            // read messages from client with header length: 4-byte size field
            while self.network_components[player].connected && connected {
                let mut size_buf = [0 as u8; 4];
                let mut size:u32 = 0;
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
                        connected = false;
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
                        match serde_json::from_str(message) {
                            Ok(value) => {
                                received_input = true;
                                ECS::combine_input(&mut input_temp, value)
                            },
                            _ => continue, // skip client if there is malformed message
                        }
                    },
                    Ok(_) => {
                        break;
                    },
                    Err(e) => {
                        eprintln!("Failed to read message for client {}: {}",self.name_components[player],e);
                        connected = false;
                    },
                }
            }

            // handle lost client
            if !connected {
                disconnected_players.push(player);
            }

            if !received_input {
                input_temp = self.player_input_components[player].clone();
                input_temp.lmb_clicked = false;
                input_temp.r_pressed = false;
            }

            // once all inputs have been aggregated for this player
            let camera = &mut self.player_camera_components[player];
            camera.rot = UnitQuaternion::from_quaternion(Quaternion::new(
                input_temp.camera_qw,
                input_temp.camera_qx,
                input_temp.camera_qy,
                input_temp.camera_qz));
            camera.camera_front = camera.rot * vector![0.0,0.0,-1.0];
            camera.camera_right = camera.rot * vector![1.0,0.0,0.0];
            camera.camera_up = camera.rot * vector![0.0,1.0,0.0];
            self.player_input_components[player] = input_temp;
        }

        // remove disconnected players
        for player in disconnected_players {
            self.handle_client_disconnect(player);
        }
    }

    /**
     * Tell all the players, the game state
     * 
     * @param   start_game: whether the game has started 
     */
    pub fn send_ready_message(&mut self, start_game: bool){
        let mut disconnected_players: Vec<Entity> = vec![];

        let lobby_ecs = self.lobby_ecs(start_game);
        let j = serde_json::to_string(&lobby_ecs).expect("Lobby ECS serialization error");
        let size = j.len() as u32 + 4;
        for &player in &self.players {
            let message = [u32::to_be_bytes(size).to_vec(), j.clone().into_bytes()].concat();
            match self.network_components[player].stream.write(&message) {
                Ok(_) => (),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
                Err(e) => {
                    eprintln!("Error updating client \"{}\": {:?}", self.name_components[player], e);
                    disconnected_players.push(player);
                }
            }
        }

        // remove players that get disconnected in lobby state
        for player in disconnected_players {
            self.remove_player(player);
        }
    }

    /**
     * Remove a player from ECS
     * 
     * @param   player key
     */
    pub fn remove_player(&mut self, player: Entity){
        self.rigid_body_set.remove(
            self.physics_components[player].handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true    // TODO: double check the bool
        );

        self.collider_set.remove(
            self.physics_components[player].collider_handle,
            &mut self.island_manager,
            &mut self.rigid_body_set,
            true    // TODO: double check the bool
        );

        self.name_components.remove(player);
        self.player_input_components.remove(player);
        self.position_components.remove(player);
        self.player_weapon_components.remove(player);
        self.model_components.remove(player);
        self.physics_components.remove(player);
        self.network_components.remove(player);
        self.player_health_components.remove(player);
        self.player_camera_components.remove(player);
        self.player_lasso_components.remove(player);
        self.player_lasso_phys_components.remove(player);
        if self.ready_players.contains_key(player) {
            self.ready_players.remove(player);
        }

        self.players.remove(self.players.iter().position(|x| *x == player).expect("not found"));
        self.dynamics.remove(self.dynamics.iter().position(|x| *x == player).expect("not found"));
        self.renderables.remove(self.renderables.iter().position(|x| *x == player).expect("not found"));
        self.active_players = self.players.len() as u8;
    }

    /**
     * Given a new player input component, updates the current component
     *
     * @param   curr: player input component to be updated
     * @param   value: player input component to be saved
     */
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
        curr.camera_qx = value.camera_qx;
        curr.camera_qy = value.camera_qy;
        curr.camera_qz = value.camera_qz;
        curr.camera_qw = value.camera_qw;
    }

    /**
     * Using TCP streams associated with each player, send updated + serialized client ECS
     */
    pub fn update_clients(&mut self) {
        let mut disconnected_players: Vec<Entity> = vec![];

        // game ends if there's 1 active player left
        if self.active_players <= 1 {
            self.game_ended = true;
        }

        let client_ecs = self.client_ecs();
        let j = serde_json::to_string(&client_ecs).expect("Client ECS serialization error");
        let size = j.len() as u32 + 4;
        for &player in &self.players {
            if self.network_components[player].connected {
                let message = [u32::to_be_bytes(size).to_vec(), j.clone().into_bytes()].concat();
                match self.network_components[player].stream.write(&message) {
                    Ok(_) => (),
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
                    Err(e) => {
                        eprintln!("Error updating client \"{}\": {:?}", self.name_components[player], e);
                        disconnected_players.push(player);
                    }
                }
            }
        }

        for player in disconnected_players {
            self.handle_client_disconnect(player);
        }
    }

    /**
     * Creates client ECS, containing names + position components of all objects and players data
     * 
     * @return  the client ECS
     */
    pub fn client_ecs(&self) -> ClientECS {
        ClientECS {
            name_components: self.name_components.clone(),
            position_components: self.position_components.clone(),
            weapon_components: self.player_weapon_components.clone(),
            model_components: self.model_components.clone(),
            health_components: self.player_health_components.clone(),
            player_lasso_components: self.player_lasso_components.clone(),
            players: self.players.clone(),
            ids: self.ids.clone(),
            renderables: self.renderables.clone(),
            game_ended: self.game_ended,
        }
    }

    /**
     * Create lobby ECS to represent the game state
     * 
     * @param   start_game: whether the game has started
     */
    pub fn lobby_ecs(&self, start_game: bool) -> LobbyECS {
        LobbyECS {
            name_components: self.name_components.clone(),
            position_components: self.position_components.clone(),
            ready_players: self.ready_players.clone(),
            players: self.players.clone(),
            ids: self.ids.clone(),
            start_game: start_game,
        }
    }

    /**
     * Creates a new player
     *
     * @param   name
     * 
     * @return  an Entity that represents the player
     */
    pub fn new_player(&mut self, name: String) -> Entity {
        let player = self.name_components.insert(name);
        self.ids.push(player);
        self.players.push(player);
        self.dynamics.push(player);
        self.renderables.push(player);
        self.model_components.insert(player, ModelComponent { modelname: "character".to_string(), scale: 1.0 });
        self.player_input_components.insert(player, PlayerInputComponent::default());
        self.player_weapon_components.insert(player, PlayerWeaponComponent::default());
        self.player_camera_components.insert(player, PlayerCameraComponent::default());
        if self.spawnpoints.is_empty() {
            eprintln!("Ran out of player spawnpoints, reusing");
            init_player_spawns(&mut self.spawnpoints);
        }
        let player_pos = self.spawnpoints.swap_remove((0..self.spawnpoints.len()).choose(&mut thread_rng()).unwrap());
        self.position_components.insert(player, PositionComponent{
            x: player_pos.translation.x,
            y: player_pos.translation.y,
            z: player_pos.translation.z,
            qx: player_pos.rotation.i,
            qy: player_pos.rotation.j,
            qz: player_pos.rotation.k,
            qw: player_pos.rotation.w,
        });
        let rigid_body = RigidBodyBuilder::dynamic().position(player_pos).lock_rotations().can_sleep(false).build();
        let handle = self.rigid_body_set.insert(rigid_body);
        let collider = ColliderBuilder::capsule_y(0.5, 0.4).user_data(player.data().as_ffi() as u128).build();
        let collider_handle = self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);
        self.physics_components.insert(player,PhysicsComponent{handle, collider_handle});
        player
    }

    pub fn spawn_prop(&mut self, name: String, modelname: String, pos_x: f32, pos_y: f32, pos_z: f32,
        roll: f32, pitch: f32, yaw: f32, dynamic: bool, shape: SharedShape, scale: f32, density: f32, restitution: f32) {
            let entity = self.name_components.insert(name);
            self.renderables.push(entity);
            let rot = UnitQuaternion::from_euler_angles(roll,pitch,yaw);
            self.position_components.insert(
                entity, 
                PositionComponent {
                    x: (pos_x),
                    y: (pos_y),
                    z: (pos_z),
                    qx: (rot.i),
                    qy: (rot.j),
                    qz: (rot.k),
                    qw: (rot.w)
                }
            );
            self.model_components.insert(entity,ModelComponent { modelname, scale });
            let rigid_body: RigidBody;
            if dynamic {
                self.dynamics.push(entity);
                rigid_body = RigidBodyBuilder::dynamic().position(
                    Isometry3::from_parts(Translation3::new(pos_x, pos_y, pos_z),rot)
                ).can_sleep(false).build();
            } else {
                rigid_body = RigidBodyBuilder::fixed().position(
                    Isometry3::from_parts(Translation3::new(pos_x, pos_y, pos_z),rot)
                ).build();
            }
            let handle = self.rigid_body_set.insert(rigid_body);
            let collider = ColliderBuilder::new(shape).density(density).restitution(restitution).user_data(
                entity.data().as_ffi() as u128
            ).build();
            let collider_handle = self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);
            self.physics_components.insert(entity, PhysicsComponent { handle, collider_handle });

    }

    /**
     * Updates position components of all objects in the game
     */
    pub fn update_positions(&mut self) {
        for &dynamic in &self.dynamics {
            let rigid_body = self.rigid_body_set.get(self.physics_components[dynamic].handle).unwrap();
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
     * Handle player firing + weapon cooldown,
     * detect if target is another player, update health components if necessary
     */
    pub fn player_fire(&mut self) {
        for &player in &self.players {
            let mut weapon = &mut self.player_weapon_components[player];
            let input = &self.player_input_components[player];
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
                if weapon.reloading && weapon.cooldown == 0 {
                    weapon.ammo = AMMO_COUNT;
                    weapon.reloading = false;
                    println!("ammo: {}",weapon.ammo);
                }
            }
            if input.lmb_clicked && weapon.cooldown == 0 && weapon.ammo > 0 {
                println!("firing!");
                let fire_vec = &self.player_camera_components[player].camera_front;
                let impulse = 10.0 * fire_vec;
                let position = &self.position_components[player];
                let halfheight = 0.5;

                let ray = Ray::new(point![position.x, position.y, position.z] + (self.player_camera_components[player].camera_up * halfheight), *fire_vec);
                let max_toi = 1000.0; //depends on size of map
                let solid = true;
                let filter = QueryFilter::new().exclude_rigid_body(self.physics_components[player].handle);
                match self.query_pipeline.cast_ray(&mut self.rigid_body_set, &mut self.collider_set, &ray, max_toi, solid, filter) {
                    Some((target_collider_handle, toi)) => {
                        let target_collider = self.collider_set.get_mut(target_collider_handle).unwrap();
                        let target = DefaultKey::from(KeyData::from_ffi(target_collider.user_data as u64));

                        let target_name = & self.name_components[target];
                        println!("Hit target {}",target_name);

                        // if target is a player, update its health component
                        if self.players.contains(&target) && self.player_health_components[target].alive {
                            self.player_health_components[target].health -= 1;
                            self.player_health_components[player].hits += 1;

                            if self.player_health_components[target].health == 0 {
                                // handle player death
                                self.player_health_components[target].alive = false;
                                self.active_players -= 1;
                                self.player_input_components[target] = PlayerInputComponent::default();
                            }
                        }

                        let hit_point = ray.point_at(toi);
                        let target_body = self.rigid_body_set.get_mut(self.physics_components[target].handle).unwrap();
                        target_body.apply_impulse_at_point(impulse, hit_point, true);

                    },
                    None => {
                        println!("Miss");
                    },
                }

                let rigid_body = self.rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
                rigid_body.apply_impulse(-impulse, true);
                // weapon cooldown is measured in ticks
                weapon.cooldown = 30;
                weapon.ammo -= 1;
                println!("ammo: {}",weapon.ammo);
            } else if (input.lmb_clicked || (input.r_pressed && weapon.ammo < AMMO_COUNT)) && weapon.cooldown == 0 {
                println!("reloading...");
                weapon.cooldown = 180;
                weapon.reloading = true;
            }
        }
    }

    /**
     * TODO: add description
     */
    pub fn player_lasso(&mut self) {
        for &player in &self.players {
            let slack = 0.01;
            let input = &self.player_input_components[player];
            if self.player_lasso_phys_components.contains_key(player) {
                let lasso_phys = &mut self.player_lasso_phys_components[player];
                if input.rmb_clicked {
                    let position = &self.position_components[player];
                    let anchor: &mut RigidBody = self.rigid_body_set.get_mut(lasso_phys.anchor_handle).unwrap();
                    let anchor_point = anchor.position() * lasso_phys.anchor_point_local;
                    self.player_lasso_components[player].anchor_x = anchor_point.x;
                    self.player_lasso_components[player].anchor_y = anchor_point.y;
                    self.player_lasso_components[player].anchor_z = anchor_point.z;
                    let dist = distance(&point![position.x, position.y, position.z],&anchor_point);
                    let new_limit = dist / 3.0_f32.sqrt() + slack;
                    let ropejoint = self.impulse_joint_set.get_mut(lasso_phys.joint_handle).unwrap();
                    if new_limit < lasso_phys.limit {
                        lasso_phys.limit = new_limit;
                    }
                    let lim = lasso_phys.limit;
                    ropejoint.data.set_limits(JointAxis::X, [lim,lim]);
                    ropejoint.data.set_limits(JointAxis::Y, [lim,lim]);
                    ropejoint.data.set_limits(JointAxis::Z, [lim,lim]);

                    let anchor = self.rigid_body_set.get_mut(self.player_lasso_phys_components[player].anchor_handle).unwrap();
                    // let anchor_t = anchor.translation().clone();
                    // TODO: calculate impulse based on mass of objects
                    anchor.apply_impulse_at_point((vector![position.x, position.y, position.z]-vector![anchor_point.x, anchor_point.y, anchor_point.z]).normalize() * 0.2, anchor_point, true);
                    let rigid_body = self.rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
                    rigid_body.apply_impulse((vector![anchor_point.x, anchor_point.y, anchor_point.z]-vector![position.x, position.y, position.z]).normalize() * 0.2, true);
                } else {
                    println!("releasing lasso");
                    self.impulse_joint_set.remove(lasso_phys.joint_handle,true);
                    self.player_lasso_phys_components.remove(player);
                    self.player_lasso_components.remove(player);
                }
            } else if input.rmb_clicked {
                println!("throwing lasso");
                let fire_vec = &self.player_camera_components[player].camera_front;
                let position = &self.position_components[player];
                let player_handle = &self.physics_components[player].handle;
                let halfheight = 0.5;

                let ray = Ray::new(point![position.x, position.y, position.z] + (self.player_camera_components[player].camera_up * halfheight), *fire_vec);
                let max_toi = 1000.0; //depends on size of map
                let solid = true;
                let filter = QueryFilter::new().exclude_rigid_body(*player_handle);
                match self.query_pipeline.cast_ray(&self.rigid_body_set, &self.collider_set, &ray, max_toi, solid, filter) {
                    Some((target_collider_handle, toi)) => {
                        let target_collider = self.collider_set.get_mut(target_collider_handle).unwrap();
                        let target = DefaultKey::from(KeyData::from_ffi(target_collider.user_data as u64));
                        let target_name = & self.name_components[target];
                        println!("Hit target {}",target_name);
                        let hit_point = ray.point_at(toi);
                        let dist = distance(&point![position.x, position.y, position.z],&hit_point);
                        let limit = dist / 3.0_f32.sqrt() + slack;
                        let target_handle = &self.physics_components[target].handle;
                        let target_body = self.rigid_body_set.get_mut(*target_handle).unwrap();
                        let hit_point_local = target_body.position().inverse() * hit_point;
                        let joint = RopeJointBuilder::new().local_anchor2(hit_point_local).limits([limit,limit]).build();
                        let joint_handle = self.impulse_joint_set.insert(*player_handle, *target_handle, joint, true);
                        self.player_lasso_phys_components.insert(player, PlayerLassoPhysComponent { anchor_handle:*target_handle, anchor_point_local: hit_point_local, joint_handle, limit });
                        self.player_lasso_components.insert(player, PlayerLassoComponent { anchor_x: hit_point.x, anchor_y: hit_point.y, anchor_z: hit_point.z });
                    },
                    None => {
                        println!("Miss");
                    },
                }
            }
        }
    }

    /**
     * TODO: add description
     */
    pub fn player_move(&mut self) {
        for &player in &self.players {
            let input = &self.player_input_components[player];
            let camera = &self.player_camera_components[player];
            let impulse = 0.05;
            let rigid_body = self.rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
            rigid_body.set_rotation(camera.rot, true);
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

    /**
     * Check for new ready updates and update number of ready players
     *
     * @param   current # of ready players
     * @return  updated # of ready players
     */
    pub fn check_ready_updates(&mut self){
        // check each connection for ready updates
        for &player in &self.players {
            let mut stream = &self.network_components[player].stream;
            let mut size_buf = [0 as u8; 4];
            match stream.peek(&mut size_buf) {
                Ok(4) => {
                    let read_size = u32::from_be_bytes(size_buf) as usize;
                    let mut read_buf = vec![0 as u8; read_size];
                    stream.read(&mut read_buf).unwrap();
                    let raw_str: &str = str::from_utf8(&read_buf[4..]).unwrap();
                    let ready_ecs: std::result::Result<ReadyECS, serde_json::Error> = serde_json::from_str(raw_str);
                    match ready_ecs {
                        Ok(ecs) => {
                            if ecs.ready {
                                self.ready_players.insert(player, ecs.ready);
                            }
                        }
                        _ => ()
                    }
                },
                _ => (),
            };
        }
        self.send_ready_message(false);
    }

    /**
     * Given a disconnected player, update network component, mark them as dead/inactive
     *
     * @param player's key
     */
    fn handle_client_disconnect(&mut self, player: DefaultKey){
        self.network_components[player].connected = false;
        self.player_health_components[player].alive = false;
        self.player_health_components[player].health = 0;
        self.active_players -= 1;
    }
}