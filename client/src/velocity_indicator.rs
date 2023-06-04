use cgmath::{Matrix4, Point3, SquareMatrix, Transform, Vector3, InnerSpace, vec3};
use crate::camera::Camera;
use crate::model::Model;
use crate::shader::Shader;
use std::{ffi::{CStr}, time::Instant};

const LERP_RATE: f32 = 50.0;

pub struct VelocityIndicator {
    model: Model,
    prev: Instant,
    vel: Vector3<f32>
}

impl VelocityIndicator {
    pub fn new() -> VelocityIndicator {
        let model = Model::new("resources/arrow/arrow.obj");
        let velocity_indicator = VelocityIndicator {
            model,
            prev: Instant::now(),
            vel: vec3(0.0, 0.0, 0.0)
        };
        velocity_indicator
    }

    pub unsafe fn draw(&mut self, camera: &Camera, velocity: Vector3<f32>, shader: &Shader) {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        self.vel = self.vel.lerp(velocity, LERP_RATE * delta);

        shader.use_program();
        let mat = camera.GetViewMatrix().invert().expect("Camera view matrix not invertible");
        let loc = mat.transform_point(Point3::new(-0.28, -0.115, -0.5));
        let mut rot_mat = Matrix4::look_at_dir(loc, self.vel, Vector3::unit_y());
        rot_mat = rot_mat.invert().expect("Velocity indicator rotation matrix not invertible");
        let sca_mat = Matrix4::from_scale((self.vel.magnitude() / 5.0).min(1.0));
        let model = rot_mat * sca_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.model.draw(shader);
    }
}