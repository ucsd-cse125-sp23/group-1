use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
type Entity = DefaultKey;

// client -> server component

#[derive(Serialize, Deserialize)]
pub struct PlayerInputComponent {
    // does this make sense to be a component?
    // we can't apply it to the ecs directly as we may receive multiple in the same tick
    pub lmb_clicked: bool,
    pub rmb_clicked: bool,
    pub w_pressed: bool,
    pub a_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
    pub camera_front_x: f32,
    pub camera_front_y: f32,
    pub camera_front_z: f32,
}

impl PlayerInputComponent {
    pub fn default() -> PlayerInputComponent{
        PlayerInputComponent{
            lmb_clicked: false,
            rmb_clicked: false,
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            camera_front_x: 0.0,
            camera_front_y: 0.0,
            camera_front_z: -1.0,
        }
    }
}

// server -> client components

#[derive(Serialize, Deserialize)]
pub struct ClientECS {
    pub name_components: SlotMap<Entity, String>,
    pub position_components: SecondaryMap<Entity, PositionComponent>,
    pub players: Vec<Entity>,
    pub temp_entity: Entity,
}

impl ClientECS {
    pub fn default() -> ClientECS{
        ClientECS{
            name_components: SlotMap::new(),
            position_components: SecondaryMap::new(),
            players: vec![],
            temp_entity: DefaultKey::default(),
        }
    }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // need rotation in unit quaternion format, unsure how to send as raw data
    // could be easier with unified linear algebra library? nalgebra works with serde (optional feature in Cargo.toml)
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
}

impl PositionComponent {
    pub fn default() -> PositionComponent{
        PositionComponent {
            x:  (0.0),
            y:  (0.0),
            z:  (0.0),
            qx: (0.0),
            qy: (0.0),
            qz: (0.0),
            qw: (1.0)
        }
    }
}

pub struct ModelComponent {
    pub modelname: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerWeaponComponent {
    pub cooldown: i16,
}