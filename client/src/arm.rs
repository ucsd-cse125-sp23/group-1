use std::{ffi::{CStr}, time::Instant};
use std::ops::Add;
use std::time::Duration;
use cgmath::{EuclideanSpace, Matrix4, Point3, Quaternion, SquareMatrix, Transform, vec3, Zero};
use cgmath::num_traits::{abs, pow};
use crate::camera::Camera;
use crate::model::Model;
use crate::shader::Shader;

const RECOIL_DURATION: f32 = 0.05;
const STABLIZE_DURATION: f32 = 0.1;

enum AnimState {
    Idle,
    Shoot,
    Reload,
}

pub struct Arm {
    model: Model,
    start_time: Instant,
    state: AnimState,
}

impl Arm {
    pub fn new() -> Arm {
        let model = Model::new("resources/arm/arm.obj");
        let arm = Arm {
            model,
            start_time: Instant::now(),
            state: AnimState::Idle,
        };
        arm
    }

    pub fn shoot(&mut self) {
        self.state = AnimState::Shoot;
        self.start_time = Instant::now();
    }

    pub fn reload(&mut self) {
        self.state = AnimState::Reload;
        self.start_time = Instant::now();
    }

    pub unsafe fn draw(&mut self, camera: &Camera, shader: &Shader) {
        let end_animation: Quaternion<f32> = Quaternion::new(0.996, 0.087, 0.0, 0.0);

        let mut rot = Quaternion::new(1.0, 0.0, 0.0, 0.0);

        let now = Instant::now();
        let time = now.duration_since(self.start_time).as_secs_f32();

        if time < RECOIL_DURATION + STABLIZE_DURATION {
            let mut lerp;
            if time < RECOIL_DURATION {
                lerp = time / RECOIL_DURATION;
                lerp = -pow(abs(lerp-1.0), 2) + 1.0;
                rot = rot.slerp(end_animation, lerp);
            } else {
                lerp = (time - RECOIL_DURATION) / STABLIZE_DURATION;
                rot = end_animation.slerp(rot, lerp);
            }
        }

        shader.use_program();
        let cam_mat = camera.GetViewMatrix().invert().expect("Camera view matrix not invertible");

        let loc_offset = cam_mat.transform_point(Point3::new(0.23, -0.2, -0.3));
        let loc_mat = Matrix4::from_translation(loc_offset.to_vec());

        let rot_mat = Matrix4::from(camera.RotQuat * rot);

        let sca_mat = Matrix4::from_scale(0.5);

        let model = loc_mat * rot_mat * sca_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.model.draw(shader);
    }
}