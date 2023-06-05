use crate::ecs::*;
use rapier3d::geometry::SharedShape;
use nalgebra::{Isometry3, Translation3, UnitQuaternion, point, Point3, Unit, vector};
use serde::Deserialize;
use std::{fs};
use std::path::Path;
use rand::{thread_rng, Rng};
use tobj;

#[derive(Deserialize)]
enum Shape {
    Ball(f32),
    Cuboid(f32,f32,f32),
    Convex(String),
    ConvexDecomp(String),
    Trimesh(String),
}

#[derive(Deserialize)]
struct Quat {
    w: f32,
    x: f32,
    y: f32,
    z: f32
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
    rot: Quat,
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
    #[serde(default = "prop_default_border")]
    border: bool,
    #[serde(default = "prop_default_max_linvel")]
    max_linvel: f32,
    #[serde(default = "prop_default_max_angvel")]
    max_angvel: f32,
}

fn prop_default_name() -> String { "UNNAMED".to_string() }
fn prop_default_modelname() -> String { "cube".to_string() }
fn prop_default_pos() -> (f32,f32,f32) { (0.0,0.0,0.0) }
fn prop_default_rot() -> Quat { Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 } }
fn prop_default_scale() -> f32 { 1.0 }
fn prop_default_dynamic() -> bool { true }
fn prop_default_shape() -> Shape { Shape::Cuboid(1.0,1.0,1.0) }
fn prop_default_density() -> f32 { 1.0 }
fn prop_default_restitution() -> f32 { 0.3 }
fn prop_default_border() -> bool { false }
fn prop_default_max_linvel() -> f32 { 0.15 }
fn prop_default_max_angvel() -> f32 { 0.1 }

#[derive(Deserialize)]
struct SpawnPoint {
    pos: (f32, f32, f32),
    #[serde(default = "spawnpoint_default_rot")]
    rot: Option<EulerRot>,
}

fn spawnpoint_default_rot() -> Option<EulerRot> { None }

fn load_scaled_model(path: &String, scale: f32) -> (Vec<Point3<f32>>, Vec<[u32; 3]>) {
    let path = Path::new(path);
    let obj = tobj::load_obj(path);
    let (models, _) = obj.unwrap();
    let mesh = &models[0].mesh;
    let num_vertices = mesh.positions.len() / 3;

    let mut vertices: Vec<Point3<f32>> = Vec::with_capacity(num_vertices);
    let indices: Vec<_> = mesh.indices.clone().chunks(3).map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32]).collect();
        
    let p = &mesh.positions;
    for i in 0..num_vertices {
        vertices.push(point!(p[i*3], p[i*3+1], p[i*3+2]) * scale);
    }

    (vertices, indices)
}

pub fn init_world(ecs: &mut ECS) {
    let j = fs::read_to_string("world/props.json").expect("Error reading file world/props.json");
    let props: Vec<Prop> = serde_json::from_str(&j).expect("Error deserializing world/props.json");
    for prop in props {
        let sharedshape = match prop.shape {
            Shape::Ball(r) => SharedShape::ball(r * prop.scale),
            Shape::Cuboid(hx, hy, hz) => SharedShape::cuboid(hx * prop.scale, hy * prop.scale, hz * prop.scale),
            Shape::Convex(path) => {
                let (vertices, _) = load_scaled_model(&path, prop.scale);
                SharedShape::convex_hull(&vertices).expect("failed to generate convex hull")
            },
            Shape::ConvexDecomp(path) => {
                // NOTE: It is assumed that scale only needs to be accurate to one decimal place
                let scale_int = (prop.scale * 10.0) as i32;
                if ecs.decomps.contains_key(&(path.clone(), scale_int)) {
                    ecs.decomps[&(path, scale_int)].clone()
                } else {
                    let (vertices, indices) = load_scaled_model(&path, prop.scale);
                    let shape = SharedShape::convex_decomposition(&vertices, &indices);
                    ecs.decomps.insert((path, scale_int), shape.clone());
                    shape
                }
            },
            Shape::Trimesh(path) => {
                let (vertices, indices) = load_scaled_model(&path, prop.scale);
                SharedShape::trimesh(vertices, indices)
            }
        };
        let linvel = if prop.max_linvel > 0.0 {
            let dir = thread_rng().gen::<UnitQuaternion<f32>>() * vector![0.0, 0.0, 1.0];
            dir * thread_rng().gen_range(0.0..prop.max_linvel)
        } else {
            vector![0.0,0.0,0.0]
        };
        let angvel = if prop.max_angvel > 0.0 {
            let angvel_axis = Unit::new_normalize(thread_rng().gen::<UnitQuaternion<f32>>() * vector![0.0, 0.0, 1.0]);
            let angvel_angle = thread_rng().gen_range(0.0..prop.max_angvel);
            let angvel_euler = UnitQuaternion::from_axis_angle(&angvel_axis, angvel_angle).euler_angles();
            vector![angvel_euler.0, angvel_euler.1, angvel_euler.2]
        } else {
            vector![0.0,0.0,0.0]
        };
        ecs.spawn_prop(
            prop.name,
            prop.modelname,
            prop.pos.0,
            prop.pos.1,
            prop.pos.2,
            prop.rot.x,
            prop.rot.y,
            prop.rot.z,
            prop.rot.w,
            prop.dynamic,
            sharedshape,
            prop.scale,
            prop.density,
            prop.restitution,
            prop.border,
            linvel,
            angvel
        );
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

#[derive(Deserialize)]
#[allow(dead_code)]
struct LoadSky {
    path: String,
    light_dir: (f32, f32, f32),
    light_diffuse: (f32, f32, f32),
    light_ambience: (f32, f32, f32)
}

pub fn init_num_skies() -> usize {
    let j = fs::read_to_string("../client/resources/skybox/skies.json").expect("Error reading file ../client/resources/skybox/skies.json");
    let loadskies: Vec<LoadSky> = serde_json::from_str(&j).expect("Error deserializing ../client/resources/skybox/skies.json");
    loadskies.len()
}