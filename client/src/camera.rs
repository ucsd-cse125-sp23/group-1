#![allow(dead_code)]
#![allow(non_snake_case)]

use std::time::Instant;

use cgmath;
use cgmath::Deg;
use cgmath::vec3;
use cgmath::prelude::*;

use crate::screenshake::ScreenShake;
use shared::*;
use shared::shared_components::PlayerInputComponent;
use crate::common::set_camera_pos;
use crate::shader::Shader;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Quaternion = cgmath::Quaternion<f32>;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM_SENSITIVITY: f32 = 0.04;
const FOV: f32 = DEFAULT_VERTICAL_FOV;
const ZOOMED_FOV: f32 = DEFAULT_VERTICAL_FOV / 2.0;
const ZOOM_RATE: f32 = 10.0 * (DEFAULT_VERTICAL_FOV - ZOOMED_FOV);
const HALFHEIGHT: f32 = 0.5;
pub struct Camera {
    // Camera Attributes
    pub Position: Point3,
    pub Front: Vector3,
    pub Up: Vector3,
    pub Right: Vector3,
    // Camera options
    pub RotQuat: Quaternion,
    pub MouseSensitivity: f32,
    pub Fov: f32,
    pub ScreenShake: ScreenShake,
    pub Prev: Instant
}
impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            Position: Point3::new(0.0, 0.0, 0.0),
            Front: vec3(0.0, 0.0, -1.0),
            Up: Vector3::zero(), // initialized later
            Right: Vector3::zero(), // initialized later
            RotQuat: Quaternion::new(1.0,0.0,0.0,0.0), // initialized later
            MouseSensitivity: SENSITIVTY,
            Fov: FOV,
            ScreenShake: ScreenShake::default(),
            Prev: Instant::now()
        };
        camera.initMatrix();
        camera
    }
}
impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        let view = Matrix4::look_at(self.Position + (self.Up * HALFHEIGHT), self.Position + (self.Up * HALFHEIGHT) + self.Front, self.Up);
        Matrix4::from(self.ScreenShake.euler) * view
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn ProcessKeyboard(&mut self, input: &PlayerInputComponent, deltaTime: f32, shader: &Shader, width: u32, height: u32) {
        let velocity = 50.0 * deltaTime;

        let mut direction: Vector3 = Vector3::zero();
        if input.d_pressed {
            direction.x += 1.0;
        }
        if input.a_pressed {
            direction.x -= 1.0;
        }
        if input.w_pressed {
            direction.z += 1.0;
        }
        if input.s_pressed {
            direction.z -= 1.0;
        }
        if input.shift_pressed {
            direction.y += 1.0;
        }
        if input.ctrl_pressed {
            direction.y -= 1.0;
        }

        direction.normalize();
        let new_pos = self.Position + velocity * (direction.x * self.Right + direction.y * self.Up + direction.z * self.Front);

        set_camera_pos(self, new_pos.to_vec(), shader, width, height);
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn ProcessMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, roll: bool) {
        xoffset *= self.MouseSensitivity;
        yoffset *= self.MouseSensitivity;
        let rot: Quaternion;
        let mut axis: Vector3;
        let mag: f32;
        if roll {
            axis = Vector3{x:yoffset,y:0.0,z:-xoffset};
        } else {
            axis = Vector3{x:yoffset,y:-xoffset,z:0.0};
        }
        mag = axis.magnitude();
        if mag != 0.0 {
            axis = (self.RotQuat * axis).normalize();

            let angle = Deg(mag);
            rot = Quaternion::from_axis_angle(axis, angle).normalize();
            self.RotQuat = (rot * self.RotQuat).normalize(); 
        }
        self.UpdateVecs();
    }

    // // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    // pub fn ProcessMouseScroll(&mut self, yoffset: f32) {
    //     if self.Fov >= 1.0 && self.Fov <= FOV {
    //         self.Fov -= yoffset;
    //     }
    //     if self.Fov <= 1.0 {
    //         self.Fov = 1.0;
    //     }
    //     if self.Fov >= FOV {
    //         self.Fov = FOV;
    //     }
    // }

    pub fn ProcessZoom(&mut self, zoomed: bool) {
        let now = Instant::now();
        let delta = now.duration_since(self.Prev).as_secs_f32();
        self.Prev = now;
        if zoomed {
            self.Fov = ZOOMED_FOV.max(self.Fov - (delta * ZOOM_RATE));
            self.MouseSensitivity = ZOOM_SENSITIVITY;
        } else {
            self.Fov = FOV.min(self.Fov + (delta * ZOOM_RATE));
            self.MouseSensitivity = SENSITIVTY;
        }
    }

    fn initMatrix(&mut self) {
        let front = vec3(0.0,0.0,1.0);
        let up = vec3(0.0,1.0,0.0);
        self.RotQuat = Quaternion::look_at( front, up).normalize();
        self.UpdateVecs();
    }

    pub fn UpdateVecs(&mut self) {
        self.Front = (self.RotQuat * vec3(0.0,0.0,-1.0)).normalize();
        self.Right = (self.RotQuat * vec3(1.0,0.0,0.0)).normalize();
        self.Up = (self.RotQuat * vec3(0.0,1.0,0.0)).normalize();
    }
}