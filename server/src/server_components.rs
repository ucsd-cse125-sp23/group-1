use nalgebra::{Vector3,UnitQuaternion,OPoint,Const};
use rapier3d::prelude::*;
use std::net::{TcpStream};

pub struct PhysicsComponent {
    pub handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
pub struct PlayerCameraComponent {
    pub rot: UnitQuaternion<f32>,
    pub camera_front: Vector3<f32>,
    pub camera_up: Vector3<f32>,
    pub camera_right: Vector3<f32>,
}

pub struct NetworkComponent {
    pub stream: TcpStream,
}

pub struct PlayerLassoPhysComponent {
    pub anchor_handle: RigidBodyHandle,
    pub anchor_point_local: OPoint<f32,Const<3>>,
    pub joint_handle: ImpulseJointHandle,
    pub limit: f32,
}