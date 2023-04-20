use nalgebra::*;
use rapier3d::prelude::*;

use self::generational_index::{GenerationalIndex, GenerationalIndexArray, GenerationalIndexAllocator};

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
    entity_allocator: GenerationalIndexAllocator,

    physics_components: EntityMap<PhysicsComponent>,
    player_camera_components: EntityMap<PlayerCameraComponent>,
    player_input_components: EntityMap<PlayerInputComponent>,
    player_weapon_components: EntityMap<PlayerWeaponComponent>,

    players: Vec<Entity>,
}

fn main() {
    let mut state = GameState{
        entity_allocator: GenerationalIndexAllocator::new(),
        physics_components: EntityMap::new(),
        player_camera_components: EntityMap::new(),
        player_input_components: EntityMap::new(),
        player_weapon_components: EntityMap::new(),
        players: vec![],
    };

    // dummy player
    let dummy_player = state.entity_allocator.allocate();
    state.player_input_components.set(dummy_player, PlayerInputComponent{
        lmb_pressed: false,
    });
    state.player_weapon_components.set(dummy_player, PlayerWeaponComponent{
        cooldown: 0,
    });
    println!("lmb_pressed: {}, cooldown: {}", state.player_input_components.get(dummy_player).unwrap().lmb_pressed, state.player_weapon_components.get(dummy_player).unwrap().cooldown);

    // loop {
    //     // physics_pipeline.step();
    // }
}