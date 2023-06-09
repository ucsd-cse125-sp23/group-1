pub mod shared_components;
pub mod shared_functions;

/** ===========================================================================
 * server settings
============================================================================ */ 

// server tick speed, in ms
// stored as 64 bit int to avoid casting for comparison
pub const TICK_SPEED: u64 = 16;
pub const MOVE_DELTA: f32 = 0.1;
pub const MIN_PLAYERS: usize = 2;
pub const AMMO_COUNT: u8 = 6;

/** ===========================================================================
 * client settings
============================================================================ */ 

// graphics settings
pub const WINDOW_TITLE: &str = "Rootin' Tootin' Spaceman Shootin' 0.2";

pub const PLAYER_CIRCLE_BORDER: f32 = 22.0;
pub const BAR_BORDER: f32 = 9.0;
pub const AMMO_BAR_BORDER: f32 = BAR_BORDER + 5.5;
pub const LEADERBOARD_SPACING: f32 = 20.0;
pub const BAR_SCALE: f32 = 0.15;
pub const PLAYER_SCALE: f32 = 0.17;
pub const CROSSHAIR_SCALE: f32 = 0.015;
pub const HITMARKER_SCALE: f32 = CROSSHAIR_SCALE * 2.0;
pub const LOBBY_BG_SCALE: f32 = 1.0;
pub const PLAYER_CIRCLE_SCALE: f32 = 0.05;
pub const WINNER_SCALE: f32 = 0.3;
pub const LEADERBOARD_SCALE: f32 = 0.5;
pub const CONTINUE_SCALE: f32 = 0.5;
pub const DEATH_MESSAGE_SCALE: f32 = 0.7;
pub const SCREEN_TXT_SCALE: f32 = 1.0;

pub const DEFAULT_VERTICAL_FOV: f32 = 59.0;

// UI element paths
pub const SPLASH_PATH: &str = "resources/ui_textures/Game-poster.jpg";

pub const LOBBY_BG_1_PATH: &str = "resources/ui_textures/lobby_bg/p1_bg_16x9.png";
pub const LOBBY_BG_2_PATH: &str = "resources/ui_textures/lobby_bg/p2_bg_16x9.png";
pub const LOBBY_BG_3_PATH: &str = "resources/ui_textures/lobby_bg/p3_bg_16x9.png";
pub const LOBBY_BG_4_PATH: &str = "resources/ui_textures/lobby_bg/p4_bg_16x9.png";

pub const GAME_OVER_BG_PATH: &str = "resources/ui_textures/game_over/space_bg.png";
pub const WINNER_TXT_PATH: &str = "resources/ui_textures/game_over/winner.png";
pub const CONTINUE_TXT_PATH: &str = "resources/ui_textures/game_over/continue-text.png";
pub const BAR_HEADER_PATH: &str = "resources/ui_textures/game_over/bar-header.png";
pub const BAR_1_PATH: &str = "resources/ui_textures/game_over/bar-1.png";
pub const BAR_2_PATH: &str = "resources/ui_textures/game_over/bar-2.png";
pub const BAR_3_PATH: &str = "resources/ui_textures/game_over/bar-3.png";
pub const BAR_4_PATH: &str = "resources/ui_textures/game_over/bar-4.png";

pub const HITS_0_PATH: &str = "resources/ui_textures/game_over/hits-0.png";
pub const HITS_1_PATH: &str = "resources/ui_textures/game_over/hits-1.png";
pub const HITS_2_PATH: &str = "resources/ui_textures/game_over/hits-2.png";
pub const HITS_3_PATH: &str = "resources/ui_textures/game_over/hits-3.png";
pub const HITS_4_PATH: &str = "resources/ui_textures/game_over/hits-4.png";
pub const HITS_5_PATH: &str = "resources/ui_textures/game_over/hits-5.png";
pub const HITS_6_PATH: &str = "resources/ui_textures/game_over/hits-6.png";

pub const P1_TXT_PATH: &str = "resources/ui_textures/game_over/p1.png";
pub const P2_TXT_PATH: &str = "resources/ui_textures/game_over/p2.png";
pub const P3_TXT_PATH: &str = "resources/ui_textures/game_over/p3.png";
pub const P4_TXT_PATH: &str = "resources/ui_textures/game_over/p4.png";

pub const P1_YOU_TXT_PATH: &str = "resources/ui_textures/game_over/p1-you.png";
pub const P2_YOU_TXT_PATH: &str = "resources/ui_textures/game_over/p2-you.png";
pub const P3_YOU_TXT_PATH: &str = "resources/ui_textures/game_over/p3-you.png";
pub const P4_YOU_TXT_PATH: &str = "resources/ui_textures/game_over/p4-you.png";

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

pub const CROSSHAIR_PATH: &str = "resources/ui_textures/crosshair/crosshair_6.png";

pub const P1_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p1-health-full.png";
pub const P2_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p2-health-full.png";
pub const P3_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p3-health-full.png";
pub const P4_HEALTH_FULL: &str = "resources/ui_textures/health_bar/p4-health-full.png";

pub const P1_HEALTH_HALF: &str = "resources/ui_textures/health_bar/p1-health-half.png";
pub const P2_HEALTH_HALF: &str = "resources/ui_textures/health_bar/p2-health-half.png";
pub const P3_HEALTH_HALF: &str = "resources/ui_textures/health_bar/p3-health-half.png";
pub const P4_HEALTH_HALF: &str = "resources/ui_textures/health_bar/p4-health-half.png";

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

pub const P1_KILL_P2_PATH: &str = "resources/ui_textures/death_messages/p1-kill-p2.png";
pub const P1_KILL_P3_PATH: &str = "resources/ui_textures/death_messages/p1-kill-p3.png";
pub const P1_KILL_P4_PATH: &str = "resources/ui_textures/death_messages/p1-kill-p4.png";
pub const P2_KILL_P1_PATH: &str = "resources/ui_textures/death_messages/p2-kill-p1.png";
pub const P2_KILL_P3_PATH: &str = "resources/ui_textures/death_messages/p2-kill-p3.png";
pub const P2_KILL_P4_PATH: &str = "resources/ui_textures/death_messages/p2-kill-p4.png";
pub const P3_KILL_P1_PATH: &str = "resources/ui_textures/death_messages/p3-kill-p1.png";
pub const P3_KILL_P2_PATH: &str = "resources/ui_textures/death_messages/p3-kill-p2.png";
pub const P3_KILL_P4_PATH: &str = "resources/ui_textures/death_messages/p3-kill-p4.png";
pub const P4_KILL_P1_PATH: &str = "resources/ui_textures/death_messages/p4-kill-p1.png";
pub const P4_KILL_P2_PATH: &str = "resources/ui_textures/death_messages/p4-kill-p2.png";
pub const P4_KILL_P3_PATH: &str = "resources/ui_textures/death_messages/p4-kill-p3.png";

pub const YOU_DIED_TXT_PATH: &str = "resources/ui_textures/screen_text/you-died.png";
pub const YOU_WIN_TXT_PATH: &str = "resources/ui_textures/screen_text/you-win.png";
pub const GAME_OVER_TXT_PATH: &str = "resources/ui_textures/screen_text/game-over.png";

pub const DAMAGE_PATH: &str = "resources/ui_textures/damage.png";
pub const HITMARKER_PATH: &str = "resources/ui_textures/crosshair/hitmarker_2.png";
pub const P1_KILLMARKER_PATH: &str = "resources/ui_textures/crosshair/killmarker_p1.png";
pub const P2_KILLMARKER_PATH: &str = "resources/ui_textures/crosshair/killmarker_p2.png";
pub const P3_KILLMARKER_PATH: &str = "resources/ui_textures/crosshair/killmarker_p3.png";
pub const P4_KILLMARKER_PATH: &str = "resources/ui_textures/crosshair/killmarker_p4.png";

/** ===========================================================================
 * audio settings
============================================================================ */
pub const AUDIO_DEBUG: bool = false; // mutes all clients but one when set to true
pub const AUDIO_FRAMES: u8 = 10; // move audio listener every N frames

// distances at which audio is full volume and inaudible, respectively
pub const ATT_MIN: f32 = 10.0;
pub const ATT_MAX: f32 = 500.;