use serde::{Deserialize, Serialize};

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

// server -> client components

#[derive(Serialize, Deserialize)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // need rotation in unit quaternion format, unsure how to send as raw data
    // could be easier with unified linear algebra library? nalgebra works with serde (optional feature in Cargo.toml)
}

#[derive(Serialize, Deserialize)]
pub struct PlayerWeaponComponent {
    pub cooldown: i8,
}