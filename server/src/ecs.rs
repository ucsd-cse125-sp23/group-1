use nalgebra::*;
use rapier3d::prelude::*;

mod generational_index;

struct PhysicsComponent {
    handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}

struct PlayerCameraComponent {
    cameraFront: Vector3<f32>,
}

struct PlayerInputComponent {
    lmb_pressed: bool,
}

struct PlayerWeaponComponent {
    cooldown: i8,
}

struct GameState {
    
}

// fn main() {
//     loop {


//         physics_pipeline.step();
//     }
// }