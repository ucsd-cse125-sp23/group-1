use crate::ecs::*;
use rapier3d::{dynamics::RigidBodySet, geometry::{ColliderSet,SharedShape}};
use nalgebra::{Isometry3, Translation3, UnitQuaternion};
use serde::Deserialize;
use std::fs;
use rand::{thread_rng, Rng};

#[derive(Deserialize)]
enum Shape {
    Ball(f32),
    Cuboid(f32,f32,f32),
    Convex{},
    ConvexDecomp{},
    Trimesh{},
}

#[derive(Deserialize)]
struct EulerRot {
    roll: f32,
    pitch: f32,
    yaw: f32,
}

#[derive(Deserialize)]
struct Prop {
    #[serde(default = "prop_default_name")]
    name: String,
    #[serde(default = "prop_default_modelname")]
    modelname: String,
    #[serde(default = "prop_default_pos")]
    pos: (f32, f32, f32),
    #[serde(default = "prop_default_rot")]
    rot: EulerRot,
    #[serde(default = "prop_default_scale")]
    scale: f32,
    #[serde(default = "prop_default_dynamic")]
    dynamic: bool,
    #[serde(default = "prop_default_shape")]
    shape: Shape,
    #[serde(default = "prop_default_density")]
    density: f32,
    #[serde(default = "prop_default_restitution")]
    restitution: f32,
}

fn prop_default_name() -> String { "UNNAMED".to_string() }
fn prop_default_modelname() -> String { "cube".to_string() }
fn prop_default_pos() -> (f32,f32,f32) { (0.0,0.0,0.0) }
fn prop_default_rot() -> EulerRot { EulerRot { roll: 0.0, pitch: 0.0, yaw: 0.0 } }
fn prop_default_scale() -> f32 { 1.0 }
fn prop_default_dynamic() -> bool { true }
fn prop_default_shape() -> Shape { Shape::Cuboid(1.0,1.0,1.0) }
fn prop_default_density() -> f32 { 1.0 }
fn prop_default_restitution() -> f32 { 0.0 }

#[derive(Deserialize)]
struct SpawnPoint {
    pos: (f32, f32, f32),
    #[serde(default = "spawnpoint_default_rot")]
    rot: Option<EulerRot>,
}

fn spawnpoint_default_rot() -> Option<EulerRot> { None }

pub fn init_world(ecs: &mut ECS, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
    let j = fs::read_to_string("world/props.json").expect("Error reading file world/props.json");
    let props: Vec<Prop> = serde_json::from_str(&j).expect("Error deserializing world/props.json");
    for prop in props {
        let sharedshape = match prop.shape {
            Shape::Ball(r) => SharedShape::ball(r * prop.scale),
            Shape::Cuboid(hx, hy, hz) => SharedShape::cuboid(hx * prop.scale, hy * prop.scale, hz * prop.scale),
            _ => panic!("Unsupported shape"),
        };
        ecs.spawn_prop(rigid_body_set, collider_set, prop.name, prop.modelname, prop.pos.0, prop.pos.1, prop.pos.2, prop.rot.roll, prop.rot.pitch, prop.rot.yaw, prop.dynamic, sharedshape, prop.scale, prop.density, prop.restitution);
    }
}

pub fn init_player_spawns(spawnpoints: &mut Vec<Isometry3<f32>>) {
    spawnpoints.clear();
    let j = fs::read_to_string("world/playerspawns.json").expect("Error reading file world/playerspawns.json");
    let spawns: Vec<SpawnPoint> = serde_json::from_str(&j).expect("Error deserializing world/playerspawns.json");
    for spawn in spawns {
        let rot = match spawn.rot {
            Some(er) => UnitQuaternion::from_euler_angles(er.roll,er.pitch,er.yaw),
            None => {
                thread_rng().gen()
            },
        };
        spawnpoints.push(Isometry3::from_parts(Translation3::new(spawn.pos.0, spawn.pos.1, spawn.pos.2), rot));
    }
}