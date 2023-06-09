// use nalgebra::*;
use rapier3d::prelude::*;
use std::collections::HashMap;
use std::{time::Duration, time::Instant};
use std::net::{TcpListener};
use polling::{Event, Poller};

mod ecs;
mod init_world;
mod server_components;
mod common;

use shared::*;
use shared::shared_functions::read_address_json;
use crate::common::*;

fn main() {
    let gravity = vector![0.0, 0.0, 0.0];
    let integration_parameters = IntegrationParameters { dt: (TICK_SPEED as f32) / 1000.0, ..Default::default()};
    let physics_hooks = ();
    let event_handler = ();

    let mut ecs = ecs::ECS::new();

    ecs.decomps = HashMap::new();
    init_world::init_world(&mut ecs);
    init_world::init_player_spawns(&mut ecs.spawnpoints);
    ecs.skies = (0..init_world::init_num_skies()).collect();
    ecs.sky = get_rand_from_vec(&mut ecs.skies);

    // connection state -- 0.0.0.0 listens to all interfaces on given port
    let (_ip, port) = read_address_json("../shared/address.json");
    let listener = TcpListener::bind("0.0.0.0:".to_string() + &port).expect("Error binding address");
    println!("[SERVER]: Waiting for at least one client...");
    ecs.connect_client(&listener);

    // poll for clients until game begins
    listener.set_nonblocking(true).unwrap();
    let key = 0;
    let poller = Poller::new().unwrap();
    // MAIN SERVER LOOP
    loop {
        poller.add(&listener, Event::readable(key)).unwrap();
        let mut events: Vec<Event> = Vec::new();

        // LOBBY LOOP
        loop {
            events.clear();
            // timeout set to server tick speed
            poller.wait(&mut events, Some(Duration::from_millis(TICK_SPEED))).unwrap();
            // connect anyone who wants to connect
            for _ in &events {
                ecs.connect_client(&listener);
                poller.modify(&listener, Event::readable(key)).unwrap();
            }
            // check each connection for ready updates
            ecs.check_ready_updates();
            // if min. # of players reached and all players are ready
            if ecs.ready_players.len() >= shared::MIN_PLAYERS && ecs.ready_players.len() == ecs.players.len() {
                ecs.send_ready_message(true);
                ecs.ready_players.clear();
                break;
            }
        }
        poller.delete(&listener).unwrap();
        ecs.update_player_models();
        // GAME LOOP
        println!("[SERVER]: Starting game");
        while !ecs.game_ended {
            // BEGIN SERVER TICK
            let start = Instant::now();

            ecs.receive_inputs();

            ecs.player_fire();
            ecs.player_lasso();
            ecs.player_move();

            ecs.update_positions();

            ecs.physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut ecs.island_manager,
                &mut ecs.broad_phase,
                &mut ecs.narrow_phase,
                &mut ecs.rigid_body_set,
                &mut ecs.collider_set,
                &mut ecs.impulse_joint_set,
                &mut ecs.multibody_joint_set,
                &mut ecs.ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );
            ecs.query_pipeline.update(&ecs.rigid_body_set, &ecs.collider_set);

            ecs.update_clients();

            // avoid playing sounds infinitely
            ecs.clear_events();

            // END SERVER TICK
            let end = Instant::now();
            // pad tick time by spin sleeping
            let tick = end.duration_since(start);
            let tick_ms = tick.as_millis() as u64;
            if tick_ms >= TICK_SPEED {
                eprintln!("ERROR: Tick took {}ms (tick speed set to {}ms)", tick_ms, TICK_SPEED);
            } else {
                spin_sleep::sleep(Duration::from_millis(TICK_SPEED) - tick);
            }
        }
        println!("[SERVER]: Game over.");

        // reset the game
        ecs.reset();
    }
}