use crate::ecs::*;
use rapier3d::{dynamics::RigidBodySet, geometry::{ColliderSet,SharedShape}};
use nalgebra::{Isometry3,Translation3,UnitQuaternion};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
enum Shape {
    Ball(f32),
    Cuboid(f32,f32,f32),
    Convex{},
    ConvexDecomp{},
    Trimesh{},
}

#[derive(Deserialize)]
struct Prop {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_modelname")]
    modelname: String,
    #[serde(default = "default_pos")]
    pos: (f32, f32, f32),
    #[serde(default = "default_rot")]
    rot: (f32, f32, f32),
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_dynamic")]
    dynamic: bool,
    #[serde(default = "default_shape")]
    shape: Shape,
    #[serde(default = "default_density")]
    density: f32,
    #[serde(default = "default_restitution")]
    restitution: f32,
}

fn default_name() -> String { "unnamed".to_string() }
fn default_modelname() -> String { "cube".to_string() }
fn default_pos() -> (f32,f32,f32) { (0.0,0.0,0.0) }
fn default_rot() -> (f32,f32,f32) { (0.0,0.0,0.0) }
fn default_scale() -> f32 { 1.0 }
fn default_dynamic() -> bool { true }
fn default_shape() -> Shape { Shape::Cuboid(1.0,1.0,1.0) }
fn default_density() -> f32 { 1.0 }
fn default_restitution() -> f32 { 0.0 }

pub fn init_world(ecs: &mut ECS, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
    let j = fs::read_to_string("world/props.json").expect("Error reading file world/props.json");
    let props: Vec<Prop> = serde_json::from_str(&j).expect("Error deserializing world/props.json");
    for prop in props {
        let sharedshape = match prop.shape {
            Shape::Ball(r) => SharedShape::ball(r * prop.scale),
            Shape::Cuboid(hx, hy, hz) => SharedShape::cuboid(hx * prop.scale, hy * prop.scale, hz * prop.scale),
            _ => panic!("Unsupported shape"),
        };
        ecs.spawn_prop(rigid_body_set, collider_set, prop.name, prop.modelname, prop.pos.0, prop.pos.1, prop.pos.2, prop.rot.0, prop.rot.1, prop.rot.2, prop.dynamic, sharedshape, prop.scale, prop.density, prop.restitution);
    }
}

pub fn init_player_spawns(ecs: &mut ECS) {
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, 5.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, -5.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(5.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(-5.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
}