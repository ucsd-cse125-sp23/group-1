mod rand;

use std::cmp::min;

use cgmath::{vec2, vec3, vec4, Vector3, Vector4, Matrix4};
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

// impl Particle {
//     pub fn new(){
//         Particle{}
//     }
// }

pub struct ParticleEmitter{
    pub particles: Vec<Particle>,
    pub particle_limit: u64,
    pub position: Vector3<f32>,
    pub frames_to_live: u8,
    pub rng: Rng,
}

impl ParticleEmitter{

    pub fn new(p_limit: u64, pos: Vector3<f32>, f_tolive: u8) -> ParticleEmitter {
        ParticleEmitter{
            particles: Vec::new(), 
            particle_limit: p_limit, 
            position: pos,
            frames_to_live: f_tolive,
            rng: rand::thread_rng(),
        }
    }

    pub fn create_particle(&self) -> Particle{
        // to think about: only generate particles going the same direction as the surface normal of the object face that it hit

        return Particle{
            frames_to_live: if self.frames_to_live == 0 {0} else {self.rng.gen_range(30..60)},               // frames to live 
            velocity: vec3(
                self.rng.gen_range(-10..10)/10., 
                self.rng.gen_range(-10..10)/10., 
                self.rng.gen_range(-10,10)/10.),         // velocity vec3
            pos: self.position,                                       // position vec3, all particles emitted from the same place
            scale: self.rng.gen_range(0..50) / 100.,                    // scaling factor
        };
    }

    pub fn getColor(p: Particle) -> Vector4<f32>{
        // this is the particle emittor that is created when the bullet hits an object
        // lets have it turn from orange to red depending on frames to live
        let max_frames_to_live = 60.;
        return vec4(1., (p.frames_to_live as f32 / max_frames_to_live), 0., 1.);
    }

    pub fn draw(&self, model: &Model, shader: &Shader) -> bool {

        // generate more particles if the particle vector isn't at full capacity
        let num_to_create = min(self.particle_limit - self.particles.len(), 2);
        for i in 0..num_to_create {
            self.particles.push(self.create_particle());
        }
        
        let mut particles_drawn = 0;

        for i in 0..self.particles.len() {
            
            // if particle is dead, generate a new one in its stead
            if (self.particles[i].frames_to_live == 0){
                self.particles[i] = self.create_particle();
            } else { self.particles[i].frames_to_live -= 1;}

            if (self.particles[i].frames_to_live == 0) {
                continue;
            }

            // update position of the particle with velocity, and the color of the particle
            self.particles[i].pos += self.particles[i].velocity;

             // setup position matrix
            let pos_mat = Matrix4::from_translation(self.particles[i].pos);

             // setup rotation matrix, right now let's just have nothing
            //  let model_qx = c_ecs.position_components[renderable].qx;
            //  let model_qy = c_ecs.position_components[renderable].qy;
            //  let model_qz = c_ecs.position_components[renderable].qz;
            //  let model_qw = c_ecs.position_components[renderable].qw;
            //  let rot_mat = Matrix4::from(Quaternion::new(model_qw, model_qx, model_qy, model_qz));
            let rot_mat = Matrix4::new_rotation(0);

            // setup scale matrix
            let scale_mat = Matrix4::from_scale(self.particles[i].scale);

            // load in the unique particle position, color into the shader
            let modelMatrix = pos_mat * scale_mat * rot_mat;

            unsafe {
                shader.set_mat4(c_str!("model"), &modelMatrix);
                shader.set_vector4(c_str!("color"), &ParticleEmitter::getColor(self, self.particles[i]));
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