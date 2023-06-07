use std::{ffi::{CStr}, time::Instant};
use std::ops::Add;
use std::time::Duration;
use cgmath::{EuclideanSpace, Matrix4, Point3, Quaternion, SquareMatrix, Transform, vec3, Zero};
use crate::camera::Camera;
use crate::model::Model;
use crate::shader::Shader;

const ANIM_DURATION: Duration = Duration::from_millis(100);

pub struct Arm {
    model: Model,
    start_time: Instant,
    rot: Quaternion<f32>,
}

impl Arm {
    pub fn new() -> Arm {
        let model = Model::new("resources/arm/arm.obj");
        let arm = Arm {
            model,
            start_time: Instant::now(),
            rot: Quaternion::zero(),
        };
        arm
    }

    pub fn shoot(&mut self) {
        self.start_time = Instant::now();
    }

    pub unsafe fn draw(&mut self, camera: &Camera, shader: &Shader) {
        let end_animation: Quaternion<f32> = Quaternion::new(0.996, 0.087, 0.0, 0.0);

        let mut rot = Quaternion::new(1.0, 0.0, 0.0, 0.0);

        let now = Instant::now();
        let time = now.duration_since(self.start_time).as_secs_f32();

        if now <= self.start_time + ANIM_DURATION {
            let mut lerp = time / ANIM_DURATION.as_secs_f32() * 2.0;
            if lerp < 1.0 {
                rot = rot.slerp(end_animation, lerp);
            } else {
                rot = end_animation.slerp(rot, lerp - 1.0);
            }
        }

        shader.use_program();
        let cam_mat = camera.GetViewMatrix().invert().expect("Camera view matrix not invertible");

        let loc_offset = cam_mat.transform_point(Point3::new(0.5, -0.5, -0.5));
        let loc_mat = Matrix4::from_translation(loc_offset.to_vec());

        let rot_mat = Matrix4::from(camera.RotQuat * rot);

        let sca_mat = Matrix4::from_scale(3.0);

        let model = loc_mat * rot_mat * sca_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.model.draw(shader);
    }
}