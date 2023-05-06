pub mod shared_components;

/** ===========================================================================
 * server settings
============================================================================ */ 

// server tick speed, in ms
// stored as 64 bit int to avoid casting for comparison
pub const TICK_SPEED: u64 = 16;
pub const MOVE_DELTA: f32 = 0.1;
pub const PORT: u32 = 2345;
pub const SERVER_ADDR: &str = "localhost";

/** ===========================================================================
 * client settings
============================================================================ */ 
pub const WINDOW_TITLE: &str = "Rootin' Tootin' Spaceman Shootin' 0.0.1";

// graphics settings
pub const SCR_WIDTH: u32 = 1000;
pub const SCR_HEIGHT: u32 = 600;