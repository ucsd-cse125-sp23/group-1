use cgmath::{Matrix4, Point3, SquareMatrix, Transform, Vector3, InnerSpace, vec3, perspective, Deg, Array};
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

    pub unsafe fn draw(&mut self, camera: &Camera, velocity: Vector3<f32>, aspect_ratio: f32, shader: &Shader) {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        self.vel = self.vel.lerp(velocity, LERP_RATE * delta);

        shader.use_program();
        let mat = camera.GetViewMatrix().invert().expect("Camera view matrix not invertible");

        let loc_offset = mat.transform_vector(vec3(-0.0, -0.0, -0.5));
        let loc_mat = Matrix4::from_translation(loc_offset);

        let loc = mat.transform_point(Point3::new(-0.0, -0.0, -0.0));
        let mut rot_mat = Matrix4::look_at_dir(loc, self.vel, Vector3::unit_y());
        rot_mat = rot_mat.invert().expect("Velocity indicator rotation matrix not invertible");

        let sca_mat = Matrix4::from_scale((self.vel.magnitude() / 5.0).min(1.0));
        let model = loc_mat * rot_mat * sca_mat;
        let model_scaleless = loc_mat * rot_mat;
        shader.set_mat4(c_str!("model"), &model);
        shader.set_mat4(c_str!("model_scaleless"), &model_scaleless);

        // offset the arrow in screen space
        let screen_mat = Matrix4::from_translation(vec3(-0.85, -0.55, 0.0));
        let mut projection: Matrix4<f32> = perspective(
            Deg(camera.Zoom),
            aspect_ratio,
            0.02,
            10000.0,
        );
        projection = screen_mat * projection;
        shader.set_mat4(c_str!("projection"), &projection);

        let light_ambience = Vector3::from_value(0.7);
        shader.setVector3(c_str!("lightAmb"), &light_ambience);

        self.model.draw(shader);
    }
}