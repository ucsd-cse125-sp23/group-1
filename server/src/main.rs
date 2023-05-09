// use nalgebra::*;
use rapier3d::prelude::*;
use std::{time::Duration, time::Instant};
use std::io::{self};
use std::net::{TcpListener};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
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

    // connection state -- 0.0.0.0 listens to all interfaces on given port
    let listener = TcpListener::bind("0.0.0.0:".to_string() + &shared::PORT.to_string()).expect("Error binding address");
    println!("[SERVER]: Waiting for at least one client...");
    ecs.connect_client(&listener, &mut rigid_body_set, &mut collider_set);

    let (tx, rx):(Sender<bool>, Receiver<bool>) = channel();
    // break upon pressing any key to start game
    thread::spawn(move || {
        println!("[SERVER]: Press ENTER to start game");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        // TODO: ready condition?
        tx.send(true).unwrap();
    });
    // meanwhile, poll for clients until game begins
    listener.set_nonblocking(true).unwrap();
    let key = 0;
    let poller = Poller::new().unwrap();
    poller.add(&listener, Event::readable(key)).unwrap();
    let mut events: Vec<Event> = Vec::new();
    loop {
        // break if game is starting
        match rx.try_recv() {
            Ok(_) => {
                ecs.send_ready_message(true);
                break;
            },
            Err(_) => {
                events.clear();
                // timeout set to server tick speed
                poller.wait(&mut events, Some(Duration::from_millis(shared::TICK_SPEED))).unwrap();
                // connect anyone who wants to connect
                for _ in &events {
                    ecs.connect_client(&listener, &mut rigid_body_set, &mut collider_set);
                    poller.modify(&listener, Event::readable(key)).unwrap();
                }
            },
        };
    }

    println!("[SERVER]: Starting game");
    loop {

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
        let tick = end.duration_since(start);
        let tick_ms = tick.as_millis() as u64;
        if tick_ms > shared::TICK_SPEED  {
            eprintln!("ERROR: Tick took {}ms (tick speed set to {}ms)", tick_ms, shared::TICK_SPEED);
        } else { 
            spin_sleep::sleep(Duration::from_millis(shared::TICK_SPEED) - tick);
        }
    }
}