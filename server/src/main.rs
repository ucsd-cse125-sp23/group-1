mod server_components;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
// use nalgebra::*;
use rapier3d::prelude::*;

use shared::shared_components::*;
use server_components::*;

type Entity = DefaultKey;

struct ECS {
    name_components: SlotMap<Entity, String>,
    // client components
    player_input_components: SecondaryMap<Entity, PlayerInputComponent>,
    position_components: SecondaryMap<Entity, PositionComponent>,
    player_weapon_components: SecondaryMap<Entity, PlayerWeaponComponent>,
    physics_components: SecondaryMap<Entity, PhysicsComponent>,
    network_components: SecondaryMap<Entity, NetworkComponent>,
    player_camera_components: SecondaryMap<Entity, PlayerCameraComponent>,

    players: Vec<Entity>,
    dynamics: Vec<Entity>,
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
        }
    }

    fn connect_client(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {

        let name = "dummy".to_string();
        let player = self.new_player(name,rigid_body_set,collider_set);
        // self.network_components.insert(player, NetworkComponent { stream: () });
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
            // let rotation = rigid_body.rotation();
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

    fn client_ecs(&self) -> ClientECS {
        ClientECS {
            name_components: self.name_components.clone(),
            position_components: self.position_components.clone(),
            players: self.players.clone(),
        }
    }
}

// fn main() {
//     let mut rigid_body_set = RigidBodySet::new();
//     let mut collider_set = ColliderSet::new();
//
//     let gravity = vector![0.0, 0.0, 0.0];
//     let integration_parameters = IntegrationParameters::default();
//     let mut physics_pipeline = PhysicsPipeline::new();
//     let mut island_manager = IslandManager::new();
//     let mut broad_phase = BroadPhase::new();
//     let mut narrow_phase = NarrowPhase::new();
//     let mut impulse_joint_set = ImpulseJointSet::new();
//     let mut multibody_joint_set = MultibodyJointSet::new();
//     let mut ccd_solver = CCDSolver::new();
//     let physics_hooks = ();
//     let event_handler = ();
//
//     let mut ecs = ECS::new();
//
//     let player = ecs.new_player("dummy".to_string(), &mut rigid_body_set, &mut collider_set);
//
//     for i in 0..200 {
//
//         if i == 10 {
//             ecs.player_input_components[player].lmb_clicked = true;
//         } else {
//
//         }
//
//         ecs.player_fire(&mut rigid_body_set);
//
//         physics_pipeline.step(
//             &gravity,
//             &integration_parameters,
//             &mut island_manager,
//             &mut broad_phase,
//             &mut narrow_phase,
//             &mut rigid_body_set,
//             &mut collider_set,
//             &mut impulse_joint_set,
//             &mut multibody_joint_set,
//             &mut ccd_solver,
//             None,
//             &physics_hooks,
//             &event_handler,
//         );
//
//         ecs.update_positions(&mut rigid_body_set);
//
//         let player_pos = &ecs.position_components[player];
//
//         println!(
//             "{}",
//             player_pos.z
//         );
//     }
// }

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

// used for any 3D value (position, velocity, acceleration)
#[derive(Serialize, Deserialize)]
struct Coords {
    x: f32,         // vec3() is f32, not f64
    y: f32,
    z: f32,
}

fn handle_client(mut stream: TcpStream) {
    let mut client_buf = [0 as u8; 256];     // using 50 byte buf
    let mut coords = Coords {x:0.0, y:0.0, z:0.0};

    while match stream.read(&mut client_buf) {
        Ok(size) => {
            // process client messages
            let message : &str = str::from_utf8(&client_buf[0..size]).unwrap();
            if message.len() > 0 {
                let value : PlayerInputComponent = serde_json::from_str(message).unwrap();

                // process keyboard input, update the new position of cube
                if value.s_pressed {
                    coords.z += -0.001;
                } else if value.w_pressed {
                    coords.z += 0.001;
                } else if value.a_pressed {
                    coords.x += -0.001;
                } else if value.d_pressed {
                    coords.x += 0.001;
                }

                // send back serialized coords to the client
                let coords_str = serde_json::to_string(&coords).unwrap();
                stream.write(coords_str.as_bytes()).unwrap();

                // debugging
                // println!("received movement: {}", value.movement);
                println!("sending coords: {}, {}, {}", coords.x, coords.y, coords.z);
            }

            // status boolean
            size > 0
        },
        Err(_) => {
            println!("An error occurred");
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;

    // accepts connections automatically
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
