#![allow(dead_code)]
#![allow(non_snake_case)]

use cgmath;
use cgmath::Deg;
use cgmath::vec3;
use cgmath::prelude::*;

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
const ZOOM: f32 = 45.0;
const HALFHEIGHT: f32 = 1.0;
pub struct Camera {
    // Camera Attributes
    pub Position: Point3,
    pub Front: Vector3,
    pub Up: Vector3,
    pub Right: Vector3,
    // Camera options
    pub RotQuat: Quaternion,
    pub MouseSensitivity: f32,
    pub Zoom: f32,
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
            Zoom: ZOOM,
        };
        camera.initMatrix();
        camera
    }
}
impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        Matrix4::look_at(self.Position + (self.Up * HALFHEIGHT), self.Position + (self.Up * HALFHEIGHT) + self.Front, self.Up)
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

    // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn ProcessMouseScroll(&mut self, yoffset: f32) {
        if self.Zoom >= 1.0 && self.Zoom <= 45.0 {
            self.Zoom -= yoffset;
        }
        if self.Zoom <= 1.0 {
            self.Zoom = 1.0;
        }
        if self.Zoom >= 45.0 {
            self.Zoom = 45.0;
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