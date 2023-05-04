#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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
pub enum Camera_Movement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
use self::Camera_Movement::*;

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera Attributes
    pub Position: Point3,
    pub Front: Vector3,
    pub Up: Vector3,
    pub Right: Vector3,
    // pub WorldUp: Vector3,
    // // Euler Angles
    // pub Yaw: f32,
    // pub Pitch: f32,
    // Camera options
    // pub MovementSpeed: f32,
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
            // WorldUp: Vector3::unit_y(),
            // Yaw: YAW,
            // Pitch: PITCH,
            // MovementSpeed: SPEED,
            RotQuat: Quaternion::new(1.0,0.0,0.0,0.0), // initialized later
            MouseSensitivity: SENSITIVTY,
            Zoom: ZOOM,
        };
        // camera.updateCameraVectors();
        camera.initMatrix();
        camera
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        Matrix4::look_at(self.Position, self.Position + self.Front, self.Up)
        // Matrix4::from(self.RotQuat) * Matrix4::from_translation(self.Position.to_vec())
    }

    // /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    // pub fn ProcessKeyboard(&mut self, direction: Camera_Movement, deltaTime: f32) {
    //     let velocity = self.MovementSpeed * deltaTime;
    //     if direction == FORWARD {
    //         self.Position += self.Front * velocity;
    //     }
    //     if direction == BACKWARD {
    //         self.Position += -(self.Front * velocity);
    //     }
    //     if direction == LEFT {
    //         self.Position += -(self.Right * velocity);
    //     }
    //     if direction == RIGHT {
    //         self.Position += self.Right * velocity;
    //     }
    // }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn ProcessMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, roll: bool) {
        xoffset *= self.MouseSensitivity;
        yoffset *= self.MouseSensitivity;
        let rot: Quaternion;
        let mut axis: Vector3;
        let mag: f32;
        if roll {
            // let rot = Quaternion::from_axis_angle(self.Front, Deg(xoffset)).normalize();
            // self.RotQuat = (rot * self.RotQuat).normalize(); 
            axis = Vector3{x:yoffset,y:0.0,z:-xoffset};
        } else {
            axis = Vector3{x:yoffset,y:-xoffset,z:0.0};
        }
        mag = axis.magnitude();
        if mag != 0.0 {
            axis = (self.RotQuat * axis).normalize();
    
            let angle = Deg(mag);
            // println!("axis: {:?}, angle: {:?}",axis,angle);

            rot = Quaternion::from_axis_angle(axis, angle).normalize();
            // println!("rot: {:?}",rot);
            self.RotQuat = (rot * self.RotQuat).normalize(); 
            // println!("RotQuat: {:?}",self.RotQuat);
        }
        self.Front = (self.RotQuat * vec3(0.0,0.0,-1.0)).normalize();
        self.Right = (self.RotQuat * vec3(1.0,0.0,0.0)).normalize();
        self.Up = (self.RotQuat * vec3(0.0,1.0,0.0)).normalize();

        // self.Yaw += xoffset;
        // self.Pitch += yoffset;

        // // Make sure that when pitch is out of bounds, screen doesn't get flipped
        // if constrainPitch {
        //     if self.Pitch > 89.0 {
        //         self.Pitch = 89.0;
        //     }
        //     if self.Pitch < -89.0 {
        //         self.Pitch = -89.0;
        //     }
        // }

        // // Update Front, Right and Up Vectors using the updated Eular angles
        // self.updateCameraVectors();
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

    // /// Calculates the front vector from the Camera's (updated) Eular Angles
    // fn updateCameraVectors(&mut self) {
    //     // Calculate the new Front vector
    //     let front = Vector3 {
    //         x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos(),
    //         y: self.Pitch.to_radians().sin(),
    //         z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos(),
    //     };
    //     self.Front = front.normalize();
    //     // Also re-calculate the Right and Up vector
    //     // TODO: Since there's no up direction in our game, we might not want to use WorldUp
    //     self.Right = self.Front.cross(self.WorldUp).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
    //     self.Up = self.Right.cross(self.Front).normalize();
    // }

    fn initMatrix(&mut self) {
        let front = vec3(0.0,0.0,1.0);
        let up = vec3(0.0,1.0,0.0);
        self.RotQuat = Quaternion::look_at( front, up).normalize();
        self.Front = (self.RotQuat * vec3(0.0,0.0,-1.0)).normalize();
        self.Right = (self.RotQuat * vec3(1.0,0.0,0.0)).normalize();
        self.Up = (self.RotQuat * vec3(0.0,1.0,0.0)).normalize();
        // println!("{:?}",self.RotQuat);
    }
}
