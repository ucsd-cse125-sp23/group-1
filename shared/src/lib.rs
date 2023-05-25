pub mod shared_components;
pub mod shared_functions;

/** ===========================================================================
 * server settings
============================================================================ */ 

// server tick speed, in ms
// stored as 64 bit int to avoid casting for comparison
pub const TICK_SPEED: u64 = 16;
pub const MOVE_DELTA: f32 = 0.1;
pub const PORT: u32 = 2345;
pub const SERVER_ADDR: &str = "localhost";
pub const MIN_PLAYERS: u8 = 2;
pub const AMMO_COUNT: u8 = 6;

/** ===========================================================================
 * client settings
============================================================================ */ 

// graphics settings
pub const WINDOW_TITLE: &str = "Rootin' Tootin' Spaceman Shootin' 0.0.1";
pub const BAR_SCALE: f32 = 0.15;
pub const PLAYER_SCALE: f32 = 0.08;
pub const CROSSHAIR_SCALE: f32 = 0.03;
pub const LOBBY_BG_SCALE: f32 = 0.2;
