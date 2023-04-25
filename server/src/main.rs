// use nalgebra::*;
use rapier3d::prelude::*;
use std::{thread::sleep,time::Duration, time::Instant};
use std::io::{self};
use std::net::{TcpListener};

use shared::shared_components::*;
mod ecs;
mod server_components;

// server tick speed, in ms
// stored as 64 bit int to avoid casting for comparison
const TICK_SPEED: u64 = 50;

const MOVE_DELTA: f32 = 0.1;

fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // let gravity = vector![0.0, 0.0, 0.0];
    // let integration_parameters = IntegrationParameters::default();
    // let mut physics_pipeline = PhysicsPipeline::new();
    // let mut island_manager = IslandManager::new();
    // let mut broad_phase = BroadPhase::new();
    // let mut narrow_phase = NarrowPhase::new();
    // let mut impulse_joint_set = ImpulseJointSet::new();
    // let mut multibody_joint_set = MultibodyJointSet::new();
    // let mut ccd_solver = CCDSolver::new();
    // let physics_hooks = ();
    // let event_handler = ();

    let mut ecs = ecs::ECS::new();

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

    let player = ecs.players[0];

    // ecs.player_input_components[player].lmb_clicked = true;

    let cube = ecs.name_components.insert("cube".to_string());
    ecs.position_components.insert(cube, PositionComponent::default());
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

        // BEGIN SERVER TICK
        let start = Instant::now();

        //temp server code
        ecs.receive_inputs();
        let input = & ecs.player_input_components[player];
        let mut position = &mut ecs.position_components[ecs.temp_entity];
        if input.s_pressed {
            position.z += -MOVE_DELTA;
        } else if input.w_pressed {
            position.z += MOVE_DELTA;
        } else if input.a_pressed {
            position.x += -MOVE_DELTA;
        } else if input.d_pressed {
            position.x += MOVE_DELTA;
        }
        // println!("sending coords: {}, {}, {}", position.x, position.y, position.z);
        ecs.update_clients();

        // END SERVER TICK
        let end = Instant::now();
        let tick = end.duration_since(start);
        let tick_ms = tick.as_millis() as u64;
        if tick_ms > TICK_SPEED  {
            eprintln!("ERROR: Tick took {}ms (tick speed set to {}ms)", tick_ms, TICK_SPEED);
        } else { 
            sleep(Duration::from_millis(TICK_SPEED) - tick);
        }
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