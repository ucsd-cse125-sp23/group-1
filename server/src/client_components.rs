use serde::{Deserialize, Serialize};

// client -> server component

#[derive(Serialize, Deserialize)]
pub struct PlayerInputComponent {
    // does this make sense to be a component?
    // we can't apply it to the ecs directly as we may receive multiple in the same tick
    lmb_clicked: bool,
    rmb_clicked: bool,
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
    camera_front_x: f32,
    camera_front_y: f32,
    camera_front_z: f32,
}

// server -> client components

#[derive(Serialize, Deserialize)]
pub struct PositionComponent {
    x: f32,
    y: f32,
    z: f32,
    // need rotation in unit quaternion format, unsure how to send as raw data
    // could be easier with unified linear algebra library? nalgebra works with serde (optional feature in Cargo.toml)
}

#[derive(Serialize, Deserialize)]
pub struct PlayerWeaponComponent {
    cooldown: i8,
}