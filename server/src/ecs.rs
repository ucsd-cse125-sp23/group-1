use nalgebra::*;
use rapier3d::prelude::*;

use self::generational_index::{GenerationalIndex, GenerationalIndexArray};

mod generational_index;

struct PhysicsComponent {
    handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}
struct PlayerCameraComponent {
    camera_front: Vector3<f32>,
}
struct PlayerInputComponent {
    lmb_pressed: bool,
}
struct PlayerWeaponComponent {
    cooldown: i8,
}

type Entity = GenerationalIndex;

type EntityMap<T> = GenerationalIndexArray<T>;

struct GameState {
    physics_components: EntityMap<PhysicsComponent>,
    player_camera_components: EntityMap<PlayerCameraComponent>,
    player_input_components: EntityMap<PlayerInputComponent>,
    player_weapon_components: EntityMap<PlayerWeaponComponent>,

    players: Vec<Entity>,
}

// fn main() {
//     loop {


//         physics_pipeline.step();
//     }
// }