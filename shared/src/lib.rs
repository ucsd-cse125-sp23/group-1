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
pub const SERVER_ADDR: &str = "127.0.0.1";
pub const MIN_PLAYERS: u8 = 2;
pub const AMMO_COUNT: u8 = 6;

/** ===========================================================================
 * client settings
============================================================================ */ 

// graphics settings
pub const WINDOW_TITLE: &str = "Rootin' Tootin' Spaceman Shootin' 0.0.1";

pub const PLAYER_CIRCLE_BORDER: f32 = 22.0;
pub const BAR_BORDER: f32 = 9.0;
pub const AMMO_BAR_BORDER: f32 = BAR_BORDER + 5.5;

pub const BAR_SCALE: f32 = 0.15;
pub const PLAYER_SCALE: f32 = 0.17;
pub const CROSSHAIR_SCALE: f32 = 0.015;
pub const LOBBY_BG_SCALE: f32 = 1.0;
pub const PLAYER_CIRCLE_SCALE: f32 = 0.05;
pub const WINNER_SCALE: f32 = 0.3;

// UI element paths
pub const SPLASH_PATH: &str = "resources/ui_textures/Game-poster.jpg";

pub const LOBBY_BG_1_PATH: &str = "resources/ui_textures/lobby_bg/p1_bg.png";
pub const LOBBY_BG_2_PATH: &str = "resources/ui_textures/lobby_bg/p2_bg.png";
pub const LOBBY_BG_3_PATH: &str = "resources/ui_textures/lobby_bg/p3_bg.png";
pub const LOBBY_BG_4_PATH: &str = "resources/ui_textures/lobby_bg/p4_bg.png";

pub const GAME_OVER_BG_PATH: &str = "resources/ui_textures/game_over/space_bg.png";
pub const WINNER_TXT_PATH: &str = "resources/ui_textures/game_over/winner.png";

pub const P1_PATH: &str = "resources/ui_textures/player_cards/p1.png";
pub const P2_PATH: &str = "resources/ui_textures/player_cards/p2.png";
pub const P3_PATH: &str = "resources/ui_textures/player_cards/p3.png";
pub const P4_PATH: &str = "resources/ui_textures/player_cards/p4.png";

pub const P1_JOINED_PATH: &str = "resources/ui_textures/player_cards/p1_joined.png";
pub const P2_JOINED_PATH: &str = "resources/ui_textures/player_cards/p2_joined.png";
pub const P3_JOINED_PATH: &str = "resources/ui_textures/player_cards/p3_joined.png";
pub const P4_JOINED_PATH: &str = "resources/ui_textures/player_cards/p4_joined.png";

pub const P1_READY_PATH: &str = "resources/ui_textures/player_cards/p1_ready.png";
pub const P2_READY_PATH: &str = "resources/ui_textures/player_cards/p2_ready.png";
pub const P3_READY_PATH: &str = "resources/ui_textures/player_cards/p3_ready.png";
pub const P4_READY_PATH: &str = "resources/ui_textures/player_cards/p4_ready.png";

pub const P1_ME_PATH: &str = "resources/ui_textures/player_cards/p1_me.png";
pub const P2_ME_PATH: &str = "resources/ui_textures/player_cards/p2_me.png";
pub const P3_ME_PATH: &str = "resources/ui_textures/player_cards/p3_me.png";
pub const P4_ME_PATH: &str = "resources/ui_textures/player_cards/p4_me.png";

pub const P1_READY_ME_PATH: &str = "resources/ui_textures/player_cards/p1_ready_me.png";
pub const P2_READY_ME_PATH: &str = "resources/ui_textures/player_cards/p2_ready_me.png";
pub const P3_READY_ME_PATH: &str = "resources/ui_textures/player_cards/p3_ready_me.png";
pub const P4_READY_ME_PATH: &str = "resources/ui_textures/player_cards/p4_ready_me.png";

pub const CROSSHAIR_PATH: &str = "resources/ui_textures/crosshair_3.png";

pub const P1_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p1-health-full.png";
pub const P2_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p2-health-full.png";
pub const P3_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p3-health-full.png";
pub const P4_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p4-health-full.png";

pub const P1_HEALTH_EMPTY: &str = "resources/ui_textures/health_bar/p1-health-empty.png";
pub const P2_HEALTH_EMPTY: &str = "resources/ui_textures/health_bar/p2-health-empty.png";
pub const P3_HEALTH_EMPTY: &str = "resources/ui_textures/health_bar/p3-health-empty.png";
pub const P4_HEALTH_EMPTY: &str = "resources/ui_textures/health_bar/p4-health-empty.png";

pub const P1_ALIVE_PATH: &str = "resources/ui_textures/player_circles/p1-circle.png";
pub const P2_ALIVE_PATH: &str = "resources/ui_textures/player_circles/p2-circle.png";
pub const P3_ALIVE_PATH: &str = "resources/ui_textures/player_circles/p3-circle.png";
pub const P4_ALIVE_PATH: &str = "resources/ui_textures/player_circles/p4-circle.png";

pub const P1_DEAD_PATH: &str = "resources/ui_textures/player_circles/p1-circle-gray.png";
pub const P2_DEAD_PATH: &str = "resources/ui_textures/player_circles/p2-circle-gray.png";
pub const P3_DEAD_PATH: &str = "resources/ui_textures/player_circles/p3-circle-gray.png";
pub const P4_DEAD_PATH: &str = "resources/ui_textures/player_circles/p4-circle-gray.png";

pub const AMMO_0_PATH: &str = "resources/ui_textures/ammo/ammo0.png";
pub const AMMO_1_PATH: &str = "resources/ui_textures/ammo/ammo1.png";
pub const AMMO_2_PATH: &str = "resources/ui_textures/ammo/ammo2.png";
pub const AMMO_3_PATH: &str = "resources/ui_textures/ammo/ammo3.png";
pub const AMMO_4_PATH: &str = "resources/ui_textures/ammo/ammo4.png";
pub const AMMO_5_PATH: &str = "resources/ui_textures/ammo/ammo5.png";
pub const AMMO_6_PATH: &str = "resources/ui_textures/ammo/ammo6.png";

pub const DAMAGE_PATH: &str = "resources/ui_textures/damage.png";
pub const HITMARKER_PATH: &str = "resources/ui_textures/hitmarker.png";

// audio settings
pub const AUDIO_DEBUG: bool = true;
pub const AUDIO_FRAMES: u8 = 10; // move audio listener every N frames