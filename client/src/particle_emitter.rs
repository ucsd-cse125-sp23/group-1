
use std::cmp::min;

use std::ffi::{CStr};

use cgmath::{vec3, vec4, Vector3, Vector4, Matrix4};
use rand::Rng;

use crate::shader::Shader;
use crate::model::Model;

pub struct Particle {
    pub frames_to_live: u8,
    pub velocity: Vector3<f32>,
    pub pos: Vector3<f32>,
    pub scale: f32,
    // pub color: Vec4<f32>, actualy color can be totally dependent on frames_to_live and the emittor itself
}

impl Particle {
    pub fn new(ftl: u8, vel: Vector3<f32>, pos: Vector3<f32>, scale: f32) -> Particle{
        Particle{
            frames_to_live: ftl,
            velocity: vel,
            pos: pos,
            scale: scale,
        }
    }
}

pub struct ParticleEmitter{
    pub particles: Vec<Particle>,
    pub particle_limit: i32,
    pub position: Vector3<f32>,
    pub frames_to_live: u8,
}

impl ParticleEmitter{

    pub fn new(p_limit: i32, pos: Vector3<f32>, f_tolive: u8) -> ParticleEmitter {
        ParticleEmitter{
            particles: Vec::new(), 
            particle_limit: p_limit, 
            position: pos,
            frames_to_live: f_tolive,
        }
    }

    pub fn create_particle(&self) -> Particle{
        // to think about: only generate particles going the same direction as the surface normal of the object face that it hit
        return Particle::new(
            if self.frames_to_live == 0 {0} else {
                rand::thread_rng().gen_range(45..60)},               // frames to live 
            vec3(
                rand::thread_rng().gen_range(-100..100) as f32/500., 
                rand::thread_rng().gen_range(-100..100) as f32/500., 
                rand::thread_rng().gen_range(-100..100) as f32/500.),         // velocity vec3
            self.position,                                       // position vec3, all particles emitted from the same place
            rand::thread_rng().gen_range(0..10) as f32/100.,                    // scaling factor
        );
    }

    // pub fn getColor(p: Particle) -> Vector4<f32>{
    //     // this is the particle emittor that is created when the bullet hits an object
    //     // lets have it turn from orange to red depending on frames to live
    //     let max_frames_to_live = 60.;
    //     return vec4(1., p.frames_to_live as f32 / max_frames_to_live, 0., 1.);
    // }

    pub fn draw(&mut self, model: &Model, shader: &Shader) -> bool {

        // generate more particles if the particle vector isn't at full capacity
        let num_to_create = min(self.particle_limit - self.particles.len() as i32, 2);
        for i in 0..num_to_create {
            self.particles.push(self.create_particle());
        }
        
        let mut particles_drawn = 0;

        for i in 0..self.particles.len() {
            
            // if particle is dead, generate a new one in its stead
            if self.particles[i].frames_to_live == 0{
                self.particles[i] = self.create_particle();
            } else { self.particles[i].frames_to_live -= 1;}

            if self.particles[i].frames_to_live == 0 {
                continue;
            }

            // update position of the particle with velocity, and the color of the particle
            let tempvel = self.particles[i].velocity;
            self.particles[i].pos += tempvel;

             // setup position matrix
            let pos_mat = Matrix4::from_translation(self.particles[i].pos);

             // setup rotation matrix
            //  let model_qx = c_ecs.position_components[renderable].qx;
            //  let model_qy = c_ecs.position_components[renderable].qy;
            //  let model_qz = c_ecs.position_components[renderable].qz;
            //  let model_qw = c_ecs.position_components[renderable].qw;
            //  let rot_mat = Matrix4::from(Quaternion::new(model_qw, model_qx, model_qy, model_qz));
            let rot_mat = Matrix4::from_scale(1.); // for now, the particles won't be rotating

            // setup scale matrix
            let scale_mat = Matrix4::from_scale(self.particles[i].scale);

            // load in the unique particle position, color into the shader
            let model_matrix = pos_mat * scale_mat * rot_mat;

            unsafe {
                shader.set_mat4(c_str!("model"), &model_matrix);
                shader.set_vector4(c_str!("color_overwrite"), 
                &vec4(1., 
                    self.particles[i].frames_to_live as f32 / 60., 
                    0., self.particles[i].frames_to_live as f32 / 120. + 0.5)
                );
            }

            // draw the model with the shader with the new position loaded in
            model.draw(shader);
            particles_drawn += 1;
        }

        if self.frames_to_live > 0 {
            self.frames_to_live -= 1;
        }

        // return false when we should remove this particle emittor
        particles_drawn > 0
    }
}