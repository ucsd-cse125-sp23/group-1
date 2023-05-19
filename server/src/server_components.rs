use nalgebra::{Vector3,UnitQuaternion};
use rapier3d::prelude::*;
use std::net::{TcpStream};

pub struct PhysicsComponent {
    pub handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle
}
pub struct PlayerCameraComponent {
    pub rot: UnitQuaternion<f32>,
    pub camera_front: Vector3<f32>,
    pub camera_up: Vector3<f32>,
    pub camera_right: Vector3<f32>
}

impl PlayerCameraComponent {
    pub fn default() -> PlayerCameraComponent{
        PlayerCameraComponent {
            rot: UnitQuaternion::identity(),
            camera_front: vector![0.0, 0.0, 0.0],
            camera_up: vector![0.0, 0.0, 0.0],
            camera_right: vector![0.0, 0.0, 0.0]
        }
    }
}

pub struct NetworkComponent {
    pub connected: bool,
    pub stream: TcpStream
}