use crate::camera::Camera;
use crate::model::Model;
use crate::shader::Shader;
use cgmath::{Deg, InnerSpace, Matrix4, perspective, SquareMatrix, Vector2, Vector3};
use std::ffi::{CStr};
use cgmath::num_traits::{abs, clamp};

pub struct ForceField {
    pub radius: f32,
    model: Model,
    shader: Shader,
    screen_size: Vector2<f32>,
}

impl ForceField {
    pub fn new(radius: f32, screen_size: Vector2<f32>) -> ForceField {
        let model = Model::new("resources/forcefield/forcefield.obj");
        let shader = Shader::new("shaders/forcefield.vs", "shaders/forcefield.fs");
        let force_field = ForceField {
            radius,
            model,
            shader,
            screen_size,
        };
        force_field
    }

    pub unsafe fn draw(&self, camera: &Camera, player_pos: Vector3<f32>) {
        self.shader.use_program();

        unsafe {
            let view = camera.GetViewMatrix();
            self.shader.set_mat4(c_str!("view"), &view);
            let projection: Matrix4<f32> = perspective(
                Deg(camera.Fov),
                self.screen_size.x / self.screen_size.y,
                0.1,
                10000.0,
            );
            self.shader.set_mat4(c_str!("projection"), &projection);
        }

        let model = Matrix4::identity();
        self.shader.set_mat4(c_str!("model"), &model);

        // alpha will be 0 at inner radius
        let show_radius = 100.0;
        let distance = abs(player_pos.magnitude() - self.radius);
        let mut alpha = distance / show_radius;
        alpha = clamp(alpha, 0.0, 1.0).powi(2);
        alpha = 1.0 - alpha;
        self.shader.set_float(c_str!("alpha"), alpha);

        self.model.draw(&self.shader);
    }
}
