// use nalgebra::*;
use rapier3d::prelude::*;
use std::{thread::sleep,time::Duration, time::Instant};
use std::io::{self};
use std::net::{TcpListener};
use std::collections::HashMap;
use config::Config;

// temporary imports, remove when temp_entity is removed
use server_components::PhysicsComponent;
use slotmap::{Key};

use shared::shared_components::*;
mod ecs;
mod init_world;
mod server_components;

fn load_settings() {
    let settings = Config::builder()
        .add_source(config::File::with_name("../shared/Settings.toml"))
        .build()
        .unwrap();
    let settings_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
}

fn main() {
    // LOAD SETTINGS
    
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
    let mut query_pipeline = QueryPipeline::new();

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

    // temp cube object
    let cube = ecs.name_components.insert("cube".to_string());
    ecs.position_components.insert(cube, PositionComponent::default());
    let cube_body = RigidBodyBuilder::dynamic().translation(vector![0.0, 0.0, 0.0]).build();
    let cube_handle = rigid_body_set.insert(cube_body);
    let cube_collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).user_data(cube.data().as_ffi() as u128).build();
    let cube_collider_handle = collider_set.insert_with_parent(cube_collider, cube_handle, &mut rigid_body_set);
    ecs.physics_components.insert(cube, PhysicsComponent { handle: (cube_handle), collider_handle: (cube_collider_handle) });
    ecs.dynamics.push(cube);

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
        ecs.receive_inputs();

        // // for each player, update position
        // // TODO: move relative to mouse orientation, switch to velocity?d
        // for player in &ecs.players {
        //     let input = & ecs.player_input_components[*player];
        //     let mut position = &mut ecs.position_components[*player];
        //     if input.s_pressed {
        //         position.z += -shared::MOVE_DELTA;
        //     } else if input.w_pressed {
        //         position.z += shared::MOVE_DELTA;
        //     } else if input.a_pressed {
        //         position.x += -shared::MOVE_DELTA;
        //     } else if input.d_pressed {
        //         position.x += shared::MOVE_DELTA;
        //     }
        // }

        ecs.player_fire(&mut rigid_body_set, &mut collider_set, &query_pipeline); 

        ecs.update_positions(&mut rigid_body_set);

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );
        query_pipeline.update(&rigid_body_set, &collider_set);

        ecs.update_clients();

        // END SERVER TICK
        let end = Instant::now();
        let tick = end.duration_since(start);
        let tick_ms = tick.as_millis() as u64;
        if tick_ms > shared::TICK_SPEED  {
            eprintln!("ERROR: Tick took {}ms (tick speed set to {}ms)", tick_ms, shared::TICK_SPEED);
        } else { 
            sleep(Duration::from_millis(shared::TICK_SPEED) - tick);
        }
    }
}