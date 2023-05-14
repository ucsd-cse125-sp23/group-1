// use nalgebra::*;
use rapier3d::prelude::*;
use std::{time::Duration, time::Instant};
use std::net::{TcpListener};
use polling::{Event, Poller};

mod ecs;
mod init_world;
mod server_components;

fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let gravity = vector![0.0, 0.0, 0.0];
    let integration_parameters = IntegrationParameters { dt: (shared::TICK_SPEED as f32) / 1000.0, ..Default::default()};
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

    init_world::init_world(&mut ecs, &mut rigid_body_set, &mut collider_set);
    init_world::init_player_spawns(&mut ecs.spawnpoints);

    // connection state -- 0.0.0.0 listens to all interfaces on given port
    let listener = TcpListener::bind("0.0.0.0:".to_string() + &shared::PORT.to_string()).expect("Error binding address");
    println!("[SERVER]: Waiting for at least one client...");
    ecs.connect_client(&listener, &mut rigid_body_set, &mut collider_set);

    // poll for clients until game begins
    listener.set_nonblocking(true).unwrap();
    let key = 0;
    // MAIN SERVER LOOP
    loop {
        let poller = Poller::new().unwrap();
        poller.add(&listener, Event::readable(key)).unwrap();
        let mut events: Vec<Event> = Vec::new();
        let mut ready_players = 0;

        // LOBBY LOOP
        loop {
            events.clear();
            // timeout set to server tick speed
            poller.wait(&mut events, Some(Duration::from_millis(shared::TICK_SPEED))).unwrap();
            // connect anyone who wants to connect
            for _ in &events {
                ecs.connect_client(&listener, &mut rigid_body_set, &mut collider_set);
                poller.modify(&listener, Event::readable(key)).unwrap();
            }
            // check each connection for ready updates
            ready_players = ecs.check_ready_updates(ready_players);
            if ready_players >= 2 && ready_players == (ecs.players.len() as u8) {
                ecs.send_ready_message(true);
                break;
            }
        }
        // GAME LOOP
        println!("[SERVER]: Starting game");
        while !ecs.game_ended {
            // BEGIN SERVER TICK
            let start = Instant::now();

            ecs.receive_inputs();

            ecs.player_fire(&mut rigid_body_set, &mut collider_set, &query_pipeline);
            ecs.player_move(&mut rigid_body_set);

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
                Some(&mut query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            ecs.update_clients();

            // END SERVER TICK
            let end = Instant::now();
            // pad tick time by spin sleeping
            let tick = end.duration_since(start);
            let tick_ms = tick.as_millis() as u64;
            if tick_ms > shared::TICK_SPEED {
                eprintln!("ERROR: Tick took {}ms (tick speed set to {}ms)", tick_ms, shared::TICK_SPEED);
            } else {
                spin_sleep::sleep(Duration::from_millis(shared::TICK_SPEED) - tick);
            }
        }
        println!("[SERVER]: Game over.");

        // reset the game
        rigid_body_set = RigidBodySet::new();
        collider_set = ColliderSet::new();

        physics_pipeline = PhysicsPipeline::new();
        island_manager = IslandManager::new();
        broad_phase = BroadPhase::new();
        narrow_phase = NarrowPhase::new();
        impulse_joint_set = ImpulseJointSet::new();
        multibody_joint_set = MultibodyJointSet::new();
        ccd_solver = CCDSolver::new();
        query_pipeline = QueryPipeline::new();

        ecs.reset(&mut rigid_body_set, &mut collider_set);
    }
}