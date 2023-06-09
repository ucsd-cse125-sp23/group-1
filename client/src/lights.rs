use std::{ffi::{CStr}, cmp::min};
use cgmath::{vec3, Vector3};
use std::time::{Instant};

use crate::shader::Shader;


const MAX_LIGHTS: usize = 8;

pub struct Light{ 
    light_dir: Vector3<f32>,
    light_amb: Vector3<f32>,
    light_dif: Vector3<f32>,
    is_point_light: bool,   // if true, light_dir stores lightPos instead
    secs_alive: f32,  // if > 0, this light dies after this many secs
    time_of_creation: Instant,
}

impl Light{
    pub fn new(light_dir: Vector3<f32>, light_amb: Vector3<f32>, 
        light_dif: Vector3<f32>,
        is_point_light: bool, secs_alive: f32) -> Light{
        Light { 
            light_dir, light_amb, light_dif,
            is_point_light, secs_alive, 
            time_of_creation: Instant::now() 
        }
    }
}

pub struct Lights{lights: Vec<Light>}
impl Lights{
    pub fn new() -> Lights{Lights{lights: Vec::new()}}
    pub fn add_light(&mut self, light: Light){
        self.lights.push(light);
    }
    pub fn clear(&mut self){self.lights.clear();}

    pub fn init_lights(&mut self, shader: &Shader, one_light: bool){

        // remove lights that outlived its life
        for i in (0..self.lights.len()).rev() {
            if self.lights[i].is_point_light {
                if self.lights[i].secs_alive <= 0. {
                    self.lights.remove(i);   
                    continue;
                }
                let now = Instant::now();
                let delta = now.duration_since(self.lights[i].time_of_creation).as_secs_f32();
                self.lights[i].secs_alive -= delta;
            }
        }
        
        // initialize variables to go into the shader
        let mut light_types = [0; MAX_LIGHTS];
        let mut light_dirs = [vec3(0.,0.,0.); MAX_LIGHTS];
        let mut light_ambs = [vec3(0.,0.,0.); MAX_LIGHTS];
        let mut light_difs = [vec3(0.,0.,0.); MAX_LIGHTS];
        let mut n = self.lights.len();
        if one_light {
            n = 1;
        }
        for i in 0..n{
            light_types[i] = if self.lights[i].is_point_light {1} else {2};
            light_dirs[i] = self.lights[i].light_dir;
            light_ambs[i] = self.lights[i].light_amb;
            light_difs[i] = self.lights[i].light_dif;
        }

        unsafe{
            shader.set_int_array(c_str!("lightType"), &light_types);
            shader.set_vector3_array(c_str!("lightDir"), &light_dirs);
            shader.set_vector3_array(c_str!("lightAmb"), &light_ambs);
            shader.set_vector3_array(c_str!("lightDif"), &light_difs);
        }
    }
}

