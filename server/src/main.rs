use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str,io,mem};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
// use nalgebra::*;
use rapier3d::prelude::*;

use shared::shared_components::*;
mod server_components;
use server_components::*;

type Entity = DefaultKey;

struct ECS {
    name_components: SlotMap<Entity, String>,
    
    // shared components
    player_input_components: SecondaryMap<Entity, PlayerInputComponent>,
    position_components: SecondaryMap<Entity, PositionComponent>,
    player_weapon_components: SecondaryMap<Entity, PlayerWeaponComponent>,
    
    // server components
    physics_components: SecondaryMap<Entity, PhysicsComponent>,
    network_components: SecondaryMap<Entity, NetworkComponent>,
    player_camera_components: SecondaryMap<Entity, PlayerCameraComponent>,

    players: Vec<Entity>,
    dynamics: Vec<Entity>,

    temp_entity: Entity,
}

impl ECS {
    fn new() -> ECS {
        ECS {
            name_components: SlotMap::new(),
            player_input_components: SecondaryMap::new(),
            position_components: SecondaryMap::new(),
            player_weapon_components: SecondaryMap::new(),
            physics_components: SecondaryMap::new(),
            network_components: SecondaryMap::new(),
            player_camera_components: SecondaryMap::new(),
            players: vec![],
            dynamics: vec![],
            temp_entity: DefaultKey::default(),
        }
    }

    fn connect_client(&mut self, listener: &TcpListener, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("Client connected: {addr:?}");
                stream.set_nonblocking(true).expect("Failed to set stream as nonblocking");
                let name = "dummy".to_string(); // TODO: get name from client
                let player = self.new_player(name.clone(),rigid_body_set,collider_set);
                self.network_components.insert(player, NetworkComponent { stream });
                println!("Name: {}", name);
            },
            Err(e) => {
                eprintln!("Failed to connect to client: {e:?}");
            },
        }
    }

    fn receive_inputs(&mut self) {
        for &player in &self.players {
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
                        break;
                    },
                    Err(e) => {
                        eprintln!("Failed to read message size for client {}: {}",self.name_components[player],e);
                        // TODO: handle lost client
                        break;
                    }
                }
                let s_size = size.try_into().unwrap();
                let mut read_buf = vec![0 as u8; s_size];
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
            self.player_input_components[player] = input_temp;
        }
    }

    // may God have mercy on our souls
    fn combine_input(input_temp: &mut PlayerInputComponent, value: PlayerInputComponent) {
        input_temp.lmb_clicked |=  value.lmb_clicked;
        input_temp.rmb_clicked |= value.rmb_clicked;
        input_temp.w_pressed |= value.w_pressed;
        input_temp.a_pressed |= value.a_pressed;
        input_temp.s_pressed |= value.s_pressed;
        input_temp.d_pressed |= value.d_pressed;
        input_temp.camera_front_x = value.camera_front_x;
        input_temp.camera_front_y = value.camera_front_y;
        input_temp.camera_front_z = value.camera_front_z;
    }

    fn update_clients(&mut self) {
        let client_ecs = self.client_ecs();
        let j = serde_json::to_string(&client_ecs).expect("Client ECS serialization error");
        let size = j.len() as u32;
        for &player in &self.players {
            let message = [u32::to_be_bytes(size).to_vec(), j.clone().into_bytes()].concat();
            match self.network_components[player].stream.write(&message) {
                Ok(_) => (),
                Err(e) => eprintln!("Error updating client \"{}\": {:?}", self.name_components[player], e)
            }
        }
    }

    fn client_ecs(&self) -> ClientECS {
        ClientECS {
            name_components: self.name_components.clone(),
            position_components: self.position_components.clone(),
            players: self.players.clone(),
            temp_entity: self.temp_entity,
        }
    }

    fn new_player(&mut self, name: String, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Entity {
        let player = self.name_components.insert(name);
        self.players.push(player);
        self.dynamics.push(player);
        self.player_input_components.insert(player, PlayerInputComponent{
            lmb_clicked: false,
            rmb_clicked: false,
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            camera_front_x: 0.0,
            camera_front_y: 0.0,
            camera_front_z: -1.0,
        });
        self.position_components.insert(player, PositionComponent{x:0.0, y:0.0, z:0.0, qx:0.0, qy:0.0, qz:0.0, qw:1.0});
        self.player_weapon_components.insert(player, PlayerWeaponComponent{cooldown: 0});
        self.player_camera_components.insert(player, PlayerCameraComponent{camera_front: vector![0.0, 0.0, -1.0]});
        let rigid_body = RigidBodyBuilder::dynamic().translation(vector![0.0, 0.0, 0.0]).build();
        let handle = rigid_body_set.insert(rigid_body);
        let collider = ColliderBuilder::capsule_y(1.0, 0.5).build();
        let collider_handle = collider_set.insert_with_parent(collider, handle, rigid_body_set);
        self.physics_components.insert(player,PhysicsComponent{handle, collider_handle});
        player
    }

    fn update_positions(&mut self, rigid_body_set: &mut RigidBodySet) {
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

    fn player_fire(&mut self, rigid_body_set: &mut RigidBodySet) {
        for &player in &self.players {
            let mut weapon = &mut self.player_weapon_components[player];
            let input = &self.player_input_components[player];
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
            }
            if input.lmb_clicked && weapon.cooldown == 0 {
                println!("firing!");
                let rigid_body = rigid_body_set.get_mut(self.physics_components[player].handle).unwrap();
                let impulse = -10.0 * self.player_camera_components[player].camera_front;
                rigid_body.apply_impulse(impulse, true);
                weapon.cooldown = 30;
            }
        }
    }
}

fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let gravity = vector![0.0, 0.0, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut ecs = ECS::new();

    // let player = ecs.new_player("dummy".to_string(), &mut rigid_body_set, &mut collider_set);

    // connection state
    let listener = TcpListener::bind("localhost:8080").expect("Error binding address");
    loop {
        println!("Waiting for client...");
        ecs.connect_client(&listener, &mut rigid_body_set, &mut collider_set);
        println!("Start game? (y/n)");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        match s.as_str() {
            "y" => break,
            _ => (),
        }
    }

    // let player = ecs.players[0];

    // ecs.player_input_components[player].lmb_clicked = true;

    let cube = ecs.name_components.insert("cube".to_string());
    ecs.position_components.insert(cube, PositionComponent { x: (0.0), y: (0.0), z: (0.0), qx: (0.0), qy: (0.0), qz: (0.0), qw: (1.0) });
    ecs.temp_entity = cube;

    loop {

        // if i == 10 {
        //     ecs.player_input_components[player].lmb_clicked = true;
        // } else {
            
        // }
        
        // ecs.player_fire(&mut rigid_body_set);    

        // physics_pipeline.step(
        //     &gravity,
        //     &integration_parameters,
        //     &mut island_manager,
        //     &mut broad_phase,
        //     &mut narrow_phase,
        //     &mut rigid_body_set,
        //     &mut collider_set,
        //     &mut impulse_joint_set,
        //     &mut multibody_joint_set,
        //     &mut ccd_solver,
        //     None,
        //     &physics_hooks,
        //     &event_handler,
        // );

        // ecs.update_positions(&mut rigid_body_set);

        // let player_pos = &ecs.position_components[player];

        // println!( "{}", player_pos.z);


        ecs.receive_inputs()
    }
}


// #[derive(Serialize, Deserialize)]
// struct ClientData {
//     client_id: u8,
//     movement: String,
// }

// // used for any 3D value (position, velocity, acceleration)
// struct Coords {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// // TODO: merge with ClientData
// struct Player {
//     position: Coords,
//     velocity: Coords,
//     hp: u8,
//     name: String,
// }

// // Dummy structure that holds movement just for now
// struct GameState {
//     players: Vec<Player>,
//     movement: String,
// }

// fn handle_client(mut stream: TcpStream) {
//     let mut client_buf = [0 as u8; 50]; // using 50 byte buf

//     // TODO: move outside of handle client function for multiple clients
//     // let mut state = GameState {
//     //     players: Vec::new(),
//     //     movement: String::from("none")
//     // };
//     // let mut dummy_player = Player {
//     //     position: Coords {x:0.0, y:0.0, z:0.0},
//     //     velocity: Coords {x:0.0, y:0.0, z:0.0},
//     //     hp: 100,
//     //     name: String::from("Dummy McDummyFace"),
//     // };
//     // state.players.push(dummy_player);

//     while match stream.read(&mut client_buf) {
//         Ok(size) => {
//             // process client messages
//             let message : &str = str::from_utf8(&client_buf[0..size]).unwrap();
//             let mut movement = "none";
//             if message.len() > 0 {
//                 let value : ClientData = serde_json::from_str(message).unwrap();
//                 println!("received: {}", value.movement);
//                 // update game state
//                 state.movement = String::from(format!("SERVER: received {}", value.movement));
//             }
//             // write game state back to client
//             // TODO: serialize state
//             let res = stream.write(state.movement.as_bytes());

//             // status boolean
//             size > 0
//         },
//         Err(_) => {
//             println!("An error occurred");
//             false
//         }
//     } {}
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("localhost:8080")?;

//     // accepts connections automatically
//     for stream in listener.incoming() {
//         handle_client(stream?);
//     }

//     /*  TODO:
//             1. Update game state
//             2. Send updated state
//             3. Wait until tick ends

//     */
//     Ok(())
// }