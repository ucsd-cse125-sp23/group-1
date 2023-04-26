/** ===========================================================================
 * server settings
============================================================================ */ 

// server tick speed, in ms
// stored as 64 bit int to avoid casting for comparison
pub const TICK_SPEED: u64 = 50;
pub const MOVE_DELTA: f32 = 0.1;


/** ===========================================================================
 * client settings
============================================================================ */ 
pub const WINDOW_TITLE: &str = "Rootin' Tootin' Spaceman Shootin' 0.0.1";

// graphics settings
pub const SCR_WIDTH: u32 = 800;
pub const SCR_HEIGHT: u32 = 600;

pub mod shared_components;