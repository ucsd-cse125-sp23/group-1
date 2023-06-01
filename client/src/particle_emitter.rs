use cgmath::{vec2, vec3};

use crate::shader::Shader;
use crate::mesh::{ Mesh, Texture, Vertex };

pub struct Particle {
    pub frames_to_live: u8,
    pub velocity: Vec3<f32>,
    pub pos: Vec3<f32>,
    pub color: Vec4<f32>
}

impl Particle {
    pub fn new(pos: Vec3<f32>, color: Vec4<f32>) -> Particle {

    }
}

pub struct ParticleEmitter{
    pub model: Model,
    pub shader: Shader,
    pub particles: Vec<Particle>,
    pub position: Vec3<f32>,
    pub particle_limit: u64,
}

// to think about: only generate particles going the same direction as the surface normal of the object face that it hit

impl ParticleEmitter{
    pub fn new(path: &str, num: u64) -> ParticleEmitter {
        let mut m = Model::default();
        m.load_model(path);
        let mut ps: Vec<Particle> = Vec::new();

        let s = Shader::new("shaders/light.vs", "shaders/light.fs");

        ParticleEmitter{model: m, shader: s, particles: ps, particle_limit: num}
    }

    pub fn draw(&mut self, shader: &Shader) {

        // generate more particles if the particle vector isn't at full capacity
        let num_to_create = min(particle_limit - particles.len(), 2);
        if (particles.len() < )
        
        for particle in self.particles {
            
            // if particle is dead, just generate a new one
            if (particle.frames)
            new_particle();

        // update position of the particle with velocity, and the color of the particle
        

        // decrease frames to live for each particle, removing ones that are dead

        // for each dead particle, create a new particle in its place
        
        // draw the particles
        

            // load in the unique particle position, scale, color into the shader

             // setup position matrix
             let pos_mat = Matrix4::from_translation(particle.position);

             // setup rotation matrix
            //  let model_qx = c_ecs.position_components[renderable].qx;
            //  let model_qy = c_ecs.position_components[renderable].qy;
            //  let model_qz = c_ecs.position_components[renderable].qz;
            //  let model_qw = c_ecs.position_components[renderable].qw;
            //  let rot_mat = Matrix4::from(Quaternion::new(model_qw, model_qx, model_qy, model_qz));
            let rot_mat = Matrix4::new_rotation(0);

             // setup scale matrix
             let scale_mat = Matrix4::from_scale(c_ecs.model_components[renderable].scale);

             let model = pos_mat * scale_mat * rot_mat;
             shader_program.set_mat4(c_str!("model"), &model);

            // draw the model with the shader with the new position loaded in
            model.draw();
        }
    }
}