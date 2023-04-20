use slotmap::{SlotMap, SecondaryMap, DefaultKey};

use nalgebra::*;
use rapier3d::prelude::*;

pub struct PhysicsComponent {
    handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}
pub struct PlayerCameraComponent {
    camera_front: Vector3<f32>,
}
pub struct PlayerInputComponent {
    lmb_pressed: bool,
}