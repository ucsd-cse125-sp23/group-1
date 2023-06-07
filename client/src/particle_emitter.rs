use std::cmp::min;
use std::ffi::{CStr};
use cgmath::{vec3, vec4, Vector3, Matrix4, Vector4, InnerSpace, Rad};
use rand::Rng;
use std::time::{Instant};
use std::f32::consts::PI;

use crate::shader::Shader;
use crate::model::Model;

pub struct Particle {
    pub secs_to_live: f32,
    pub velocity: Vector3<f32>,
    pub position: Vector3<f32>,
    pub rotation: Matrix4<f32>,
    pub scale: f32,
}

#[derive(Clone)]
pub struct ParticleEmitterSpecifier{
    pub stl_min: f32, pub stl_max: f32,     // particles: seconds to live
    pub scl_min: f32, pub scl_max: f32,     // particles: scale
    pub phi_max: f32,                       // particles: velocity
    pub col_start: Vector4<f32>,            // particles: color
    pub col_end: Vector4<f32>,
    pub particle_limit: i32,
    pub secs_to_live: f32,                  // for the emitter
    pub particles_per_100ms: i32,           
}

pub struct ParticleEmitter{
    pub particles: Vec<Particle>,
    pub position: Vector3<f32>,
    pub vars: ParticleEmitterSpecifier,
    prev: Instant,                          // previous frame
    prev_incr: Instant,                     // to count 100ms increments
    time_alive: f32,                        // to know when to stop making new particles
    s_normal: Vector3<f32>,                 // surface normal of the face that is hit
    start_vel: Vector3<f32>                 // velocity of the object that was hit
}

impl ParticleEmitter{

    pub fn new(pos: Vector3<f32>, s_normal: Vector3<f32>, start_vel: Vector3<f32>, pe_specifier: &ParticleEmitterSpecifier) -> ParticleEmitter {

        ParticleEmitter{
            particles: Vec::new(),  
            position: pos,
            vars: pe_specifier.clone(),
            prev: Instant::now(),
            prev_incr: Instant::now(),
            time_alive: 0.,
            s_normal,
            start_vel
        }
        
    }

    // returns particle with secs_to_live = 0 if emitter time is up
    pub fn create_particle(&self) -> Particle{
        
        // generate around the z axis
        let theta: f32 = rand::thread_rng().gen_range(0.0..(2.*PI));
        let phi: f32 = rand::thread_rng().gen_range(0.0..self.vars.phi_max);
        let vel = vec3(
            phi.sin()*theta.cos(), 
            phi.sin()*theta.sin(), 
            phi.cos());

        // create new coordinate system around the surface normal
        let n = self.s_normal;
        let mut a = vec3(0., 0., 1.);
        if n.z.abs() > 0.9 {
            a = vec3(1., 0., 0.);
        }
        let b = a.cross(self.s_normal).normalize();
        let c = n.cross(b);
        let mut new_vel = vel.x*b + vel.y*c + vel.z*n;
        // TODO: get particle velocity magnitude from ParticleEmitterSpecifier, possibly generate random
        new_vel = new_vel * 6.0 + self.start_vel;

        let theta: f32 = rand::thread_rng().gen_range(0.0..(2.*PI));
        let phi: f32 = rand::thread_rng().gen_range(0.0..PI);
        let axis = vec3(
            phi.sin()*theta.cos(), 
            phi.sin()*theta.sin(), 
            phi.cos());
        let angle = Rad(rand::thread_rng().gen_range(0.0..PI));

        let rot = Matrix4::from_axis_angle(axis, angle);

        return Particle { 
            secs_to_live: if self.time_alive >= self.vars.secs_to_live {0.0} else
                {rand::thread_rng().gen_range(self.vars.stl_min..self.vars.stl_max)}, 
            velocity: new_vel, 
            position: self.position, 
            rotation: rot,
            scale: rand::thread_rng().gen_range(self.vars.scl_min..self.vars.scl_max) 
        }
    }
    
    pub fn draw(&mut self, model: &Model, shader: &Shader) -> bool {

        // generate more particles if the particle vector isn't at full capacity
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        self.time_alive += delta;

        if now.duration_since(self.prev_incr).as_secs_f32() >= 0.1 {
            let num_to_create = min(self.vars.particles_per_100ms,
                self.vars.particle_limit - self.particles.len() as i32);
            for _ in 0..num_to_create {
                self.particles.push(self.create_particle());
            }
            self.prev_incr = now;
        }

        self.position += delta * self.start_vel;
        
        // update position/color, and draw each particle
        let mut particles_drawn = 0;

        for i in 0..self.particles.len() {
            
            // if particle is dead, generate a new one in its stead
            if self.particles[i].secs_to_live <= 0.0 {
                self.particles[i] = self.create_particle();
            } else { self.particles[i].secs_to_live -= delta;}

            // skip drawing this particle if the emitter time is up
            if self.particles[i].secs_to_live <= 0.0 {
                continue;
            }

            // update position of the particle with velocity, and the color of the particle
            let tempvel = self.particles[i].velocity;
            self.particles[i].position += tempvel * delta;

            // setup model matrix
            let pos_mat = Matrix4::from_translation(self.particles[i].position);
            let rot_mat = self.particles[i].rotation;
            let scale_mat = Matrix4::from_scale(self.particles[i].scale);
            let model_matrix = pos_mat * scale_mat * rot_mat;

            unsafe {
                shader.set_mat4(c_str!("model"), &model_matrix);
                shader.set_vector4(c_str!("color_overwrite"), 
                &vec4(1., 
                    self.particles[i].secs_to_live / self.vars.stl_max, 
                    0., 1.)
                );
            }

            // draw the model with the shader with the new position loaded in
            model.draw(shader);
            particles_drawn += 1;
        }

        // return true when we should remove this particle emittor
        self.time_alive >= self.vars.secs_to_live && particles_drawn == 0
    }

    
}