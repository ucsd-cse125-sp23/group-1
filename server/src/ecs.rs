use nalgebra::*;
use rapier3d::prelude::*;

// mod generational_index;
// use self::generational_index::{GenerationalIndex, GenerationalIndexArray, GenerationalIndexAllocator};

use slotmap::{SlotMap, SecondaryMap, DefaultKey};

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

// type Entity = GenerationalIndex;
type Entity = DefaultKey;

// type EntityMap<T> = GenerationalIndexArray<T>;

struct GameState {
    // entity_allocator: GenerationalIndexAllocator,
    entity_allocator: SlotMap<DefaultKey, ()>,

    physics_components: SecondaryMap<DefaultKey, PhysicsComponent>,
    player_camera_components: SecondaryMap<DefaultKey, PlayerCameraComponent>,
    player_input_components: SecondaryMap<DefaultKey, PlayerInputComponent>,
    player_weapon_components: SecondaryMap<DefaultKey, PlayerWeaponComponent>,

    players: Vec<Entity>,
}

// fn main() {
//     let mut state = GameState{
//         entity_allocator: GenerationalIndexAllocator::new(),
//         physics_components: EntityMap::new(),
//         player_camera_components: EntityMap::new(),
//         player_input_components: EntityMap::new(),
//         player_weapon_components: EntityMap::new(),
//         players: vec![],
//     };

//     // dummy player
//     let dummy_player = state.entity_allocator.allocate();
//     state.player_input_components.set(dummy_player, PlayerInputComponent{
//         lmb_pressed: false,
//     });
//     state.player_weapon_components.set(dummy_player, PlayerWeaponComponent{
//         cooldown: 0,
//     });
//     println!("lmb_pressed: {}, cooldown: {}", state.player_input_components.get(dummy_player).unwrap().lmb_pressed, state.player_weapon_components.get(dummy_player).unwrap().cooldown);

//     // loop {
//     //     // physics_pipeline.step();
//     // }
// }