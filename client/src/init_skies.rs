use cgmath::{Vector3, vec3, InnerSpace};
use serde::Deserialize;
use std::fs;

use crate::skybox::Skybox;

#[derive(Deserialize)]
struct LoadSky {
    path: String,
    light_dir: (f32, f32, f32),
    light_diffuse: (f32, f32, f32),
    light_ambience: (f32, f32, f32)
}

pub struct Sky {
    pub skybox: Skybox,
    pub light_dir: Vector3<f32>,
    pub light_diffuse: Vector3<f32>,
    pub light_ambience: Vector3<f32>,
}

pub fn init_skyboxes() -> Vec<Sky> {
    let mut skies = vec![];
    let j = fs::read_to_string("resources/skybox/skies.json").expect("Error reading file resources/skybox/skies.json");
    let loadskies: Vec<LoadSky> = serde_json::from_str(&j).expect("Error deserializing resources/skybox/skies.json");
    for loadsky in loadskies {
        skies.push(Sky {
            skybox: unsafe{ Skybox::new(&loadsky.path, ".png") },
            light_dir: vec3(loadsky.light_dir.0, loadsky.light_dir.1, loadsky.light_dir.2).normalize(),
            light_diffuse: vec3(loadsky.light_diffuse.0, loadsky.light_diffuse.1, loadsky.light_diffuse.2),
            light_ambience: vec3(loadsky.light_ambience.0, loadsky.light_ambience.1, loadsky.light_ambience.2) 
        });
    }
    skies
}