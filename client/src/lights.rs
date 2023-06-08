use std::ffi::{CStr};
use cgmath::{vec3, Vector3};
use std::time::{Instant};

use crate::shader::Shader;


const MAX_LIGHTS: i32 = 4;

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
        light_dif: Vector3<f32>, is_point_light: bool, secs_alive: f32) -> Light{
        Light { 
            light_dir, light_amb, light_dif, is_point_light, secs_alive, 
            time_of_creation: Instant::now() 
        }
    }
}

pub struct Lights{
    lights: Vec<Light>,
    light_dirs: Vec<Vector3<f32>>,
    light_ambs: Vec<Vector3<f32>>,
    light_difs: Vec<Vector3<f32>>,
    light_types: Vec<Vector3<i32>>,
}

impl Lights{

    pub fn new() -> Lights{
        Lights{
            lights: Vec::new(),
            light_dirs: Vec::new(),
            light_ambs: Vec::new(),
            light_difs: Vec::new(),
            light_types: Vec::new(),
        }
    }

    pub fn addLight(&mut self, light: Light){
        self.lights.push(light);
        self.light_dirs.push(light.light_dir);
        self.light_ambs.push(light.light_amb);
        self.light_difs.push(light.light_dif);
        self.light_types.push(light.light_type);
    }

    pub fn clear(&mut self){
        self.lights.clear();
    }

    pub fn init_lights(&mut self, shader: &Shader){

        let n = self.lights.len();
        let mut ind = 0;

        // init lighting vars in shader
        for i in (0..n).rev() {
            if self.lights[i].is_point_light {

                // remove this light if outlived its life
                let now = Instant::now();
                let delta = now.duration_since(self.lights[i].time_of_creation).as_secs_f32();
                self.lights[i].secs_alive -= delta;

                if self.lights[i].secs_alive <= 0. {
                    self.lights.remove(i);   
                    continue;
                }
            }
            unsafe{
                shader.set_int(c_str!("lightType"), if self.lights[i].is_point_light {1} else {2});
                shader.set_vector3(c_str!("lightDir"), &self.lights[i].light_dir);
                shader.set_vector3(c_str!("lightAmb"), &self.lights[i].light_amb);
                shader.set_vector3(c_str!("lightDif"), &self.lights[i].light_dif);
            }
            ind += 1;
        }
        
        // clear the rest of the lights in the shader from before
        for i in ind..n {
            unsafe{
                shader.set_int(c_str!("lightType"), 0);
            }
        }
    }
}

