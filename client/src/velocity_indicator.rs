use cgmath::{Matrix4, Point3, SquareMatrix, Transform, Vector3};
use crate::camera::Camera;
use crate::model::Model;
use crate::shader::Shader;
use std::ffi::{CStr};

pub struct VelocityIndicator {
    model: Model,
}

impl VelocityIndicator {
    pub fn new() -> VelocityIndicator {
        let model = Model::new("resources/arrow/arrow.obj");
        let velocity_indicator = VelocityIndicator {
            model,
        };
        velocity_indicator
    }

    pub unsafe fn draw(&self, camera: &Camera, velocity: Vector3<f32>, shader: &Shader) {
        shader.use_program();
        let mat = camera.GetViewMatrix().invert().expect("Camera view matrix not invertible");
        let loc = mat.transform_point(Point3::new(-0.28, -0.115, -0.5));
        let mut rot_mat = Matrix4::look_at_dir(loc, velocity, Vector3::unit_y());
        rot_mat = rot_mat.invert().expect("Velocity indicator rotation matrix not invertible");
        let model = rot_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.model.draw(shader);
    }
}