use crate::fadable::Fadable;
use crate::sprite_renderer::{Anchor, Sprite};
use cgmath::{vec2, Vector2};
use shared::*;
use shared::shared_components::*;
use slotmap::DefaultKey;

pub struct UI {
    // ========================== splash ui elements ==========================
    pub splash: Sprite,
  
    // ========================== lobby ui elements ===========================
    pub p1_lobby: Sprite,
    pub p2_lobby: Sprite,
    pub p3_lobby: Sprite,
    pub p4_lobby: Sprite,
    pub p1: Sprite,
    pub p2: Sprite,
    pub p3: Sprite,
    pub p4: Sprite,
    pub p1_joined: Sprite,
    pub p2_joined: Sprite,
    pub p3_joined: Sprite,
    pub p4_joined: Sprite,
    pub p1_ready: Sprite,
    pub p2_ready: Sprite,
    pub p3_ready: Sprite,
    pub p4_ready: Sprite,
    pub p1_me: Sprite,
    pub p2_me: Sprite,
    pub p3_me: Sprite,
    pub p4_me: Sprite,
    pub p1_ready_me: Sprite,
    pub p2_ready_me: Sprite,
    pub p3_ready_me: Sprite,
    pub p4_ready_me: Sprite,

    // =========================== game ui elements ===========================
    pub crosshair: Sprite,

    pub p1_healthbar: Sprite,
    pub p2_healthbar: Sprite,
    pub p3_healthbar: Sprite,
    pub p4_healthbar: Sprite,

    pub p1_halfbar: Sprite,
    pub p2_halfbar: Sprite,
    pub p3_halfbar: Sprite,
    pub p4_halfbar: Sprite,

    pub p1_emptybar: Sprite,
    pub p2_emptybar: Sprite,
    pub p3_emptybar: Sprite,
    pub p4_emptybar: Sprite,

    pub ammo_0: Sprite,
    pub ammo_1: Sprite,
    pub ammo_2: Sprite,
    pub ammo_3: Sprite,
    pub ammo_4: Sprite,
    pub ammo_5: Sprite,
    pub ammo_6: Sprite,

    pub p1_alive: Sprite,
    pub p2_alive: Sprite,
    pub p3_alive: Sprite,
    pub p4_alive: Sprite,
    
    pub p1_dead: Sprite,
    pub p2_dead: Sprite,
    pub p3_dead: Sprite,
    pub p4_dead: Sprite,

    pub p1_kill_p2: Sprite,
    pub p1_kill_p3: Sprite,
    pub p1_kill_p4: Sprite,
    pub p2_kill_p1: Sprite,
    pub p2_kill_p3: Sprite,
    pub p2_kill_p4: Sprite,
    pub p3_kill_p1: Sprite,
    pub p3_kill_p2: Sprite,
    pub p3_kill_p4: Sprite,
    pub p4_kill_p1: Sprite,
    pub p4_kill_p2: Sprite,
    pub p4_kill_p3: Sprite,

    pub damage: Fadable,
    pub hitmarker: Fadable,

    // ======================== game over ui elements =========================
    pub game_over_bg: Sprite,
    pub winner_txt: Sprite,
    pub continue_txt: Sprite,
    pub p1_winner: Sprite,
    pub p2_winner: Sprite,
    pub p3_winner: Sprite,
    pub p4_winner: Sprite,

    pub bar_header: Sprite,
    pub bar_1: Sprite,
    pub bar_2: Sprite,
    pub bar_3: Sprite,
    pub bar_4: Sprite,

    pub hits_0_b1: Sprite,
    pub hits_1_b1: Sprite,
    pub hits_2_b1: Sprite,
    pub hits_3_b1: Sprite,
    pub hits_4_b1: Sprite,
    pub hits_5_b1: Sprite,
    pub hits_6_b1: Sprite,
    
    pub hits_0_b2: Sprite,
    pub hits_1_b2: Sprite,
    pub hits_2_b2: Sprite,
    pub hits_3_b2: Sprite,
    pub hits_4_b2: Sprite,
    pub hits_5_b2: Sprite,
    pub hits_6_b2: Sprite,

    pub hits_0_b3: Sprite,
    pub hits_1_b3: Sprite,
    pub hits_2_b3: Sprite,
    pub hits_3_b3: Sprite,
    pub hits_4_b3: Sprite,
    pub hits_5_b3: Sprite,
    pub hits_6_b3: Sprite,

    pub hits_0_b4: Sprite,
    pub hits_1_b4: Sprite,
    pub hits_2_b4: Sprite,
    pub hits_3_b4: Sprite,
    pub hits_4_b4: Sprite,
    pub hits_5_b4: Sprite,
    pub hits_6_b4: Sprite,

    pub p1_text_b1: Sprite,
    pub p2_text_b1: Sprite,
    pub p3_text_b1: Sprite,
    pub p4_text_b1: Sprite,
    
    pub p1_text_b2: Sprite,
    pub p2_text_b2: Sprite,
    pub p3_text_b2: Sprite,
    pub p4_text_b2: Sprite,

    pub p1_text_b3: Sprite,
    pub p2_text_b3: Sprite,
    pub p3_text_b3: Sprite,
    pub p4_text_b3: Sprite,

    pub p1_text_b4: Sprite,
    pub p2_text_b4: Sprite,
    pub p3_text_b4: Sprite,
    pub p4_text_b4: Sprite,

    pub p1_you_text: Sprite,
    pub p2_you_text: Sprite,
    pub p3_you_text: Sprite,
    pub p4_you_text: Sprite
}

impl UI {
    pub fn initialize(s_size: Vector2<f32>, id: u32, width: f32, height: f32) -> UI {

        // set up UI element positions
        let bg_pos = vec2(width / 2.0, height / 2.0);
        let p1_pos = vec2(width / 5.00, height / 2.0);
        let p2_pos = vec2(width / 2.50, height / 2.0);
        let p3_pos = vec2(width / 1.67, height / 2.0);
        let p4_pos = vec2(width / 1.25, height / 2.0);

        let c1_pos = vec2(width * (7.0 / 20.0), height - PLAYER_CIRCLE_BORDER);
        let c2_pos = vec2(width * (9.0 / 20.0), height - PLAYER_CIRCLE_BORDER);
        let c3_pos = vec2(width * (11.0 / 20.0), height - PLAYER_CIRCLE_BORDER);
        let c4_pos = vec2(width * (13.0 / 20.0), height - PLAYER_CIRCLE_BORDER);

        let winner_pos = vec2(width / 4.2, height / 2.4);
        let winner_txt_pos = vec2(width / 4.2, height / 1.16);
        let continue_pos = vec2(width / 1.45, height / 5.0);
        let bar_header_pos = vec2(width / 1.45, height / 1.3 - LEADERBOARD_SPACING * 1.0);

        let bar_1_pos = vec2(width / 1.45, height / 1.3 - LEADERBOARD_SPACING * 2.5);
        let bar_2_pos = vec2(width / 1.45, height / 1.3 - LEADERBOARD_SPACING * 4.0);
        let bar_3_pos = vec2(width / 1.45, height / 1.3 - LEADERBOARD_SPACING * 5.5);
        let bar_4_pos = vec2(width / 1.45, height / 1.3 - LEADERBOARD_SPACING * 7.0);

        let health_pos = vec2(BAR_BORDER, BAR_BORDER);
        let ammo_pos = vec2(width - BAR_BORDER, AMMO_BAR_BORDER);
        let death_message_pos = vec2(width / 2.0, height / 1.25);

        UI {
            // ============================ splash screen =============================
            splash: init_sprite(s_size, id, SPLASH_PATH, bg_pos, LOBBY_BG_SCALE),
            // =========================== lobby background ===========================
            p1_lobby: init_sprite(s_size, id, LOBBY_BG_1_PATH, bg_pos, LOBBY_BG_SCALE),
            p2_lobby: init_sprite(s_size, id, LOBBY_BG_2_PATH, bg_pos, LOBBY_BG_SCALE),
            p3_lobby: init_sprite(s_size, id, LOBBY_BG_3_PATH, bg_pos, LOBBY_BG_SCALE),
            p4_lobby: init_sprite(s_size, id, LOBBY_BG_4_PATH, bg_pos, LOBBY_BG_SCALE),
            
            // =========================== gray player cards ==========================
            p1: init_sprite(s_size, id, P1_PATH, p1_pos, PLAYER_SCALE),
            p2: init_sprite(s_size, id, P2_PATH, p2_pos, PLAYER_SCALE),
            p3: init_sprite(s_size, id, P3_PATH, p3_pos, PLAYER_SCALE),
            p4: init_sprite(s_size, id, P4_PATH, p4_pos, PLAYER_SCALE),

            // ====================== client joined player cards ======================
            p1_joined: init_sprite(s_size, id, P1_JOINED_PATH, p1_pos, PLAYER_SCALE),
            p2_joined: init_sprite(s_size, id, P2_JOINED_PATH, p2_pos, PLAYER_SCALE),
            p3_joined: init_sprite(s_size, id, P3_JOINED_PATH, p3_pos, PLAYER_SCALE),
            p4_joined: init_sprite(s_size, id, P4_JOINED_PATH, p4_pos, PLAYER_SCALE),

            // ========================== ready player cards ==========================
            p1_ready: init_sprite(s_size, id, P1_READY_PATH, p1_pos, PLAYER_SCALE),
            p2_ready: init_sprite(s_size, id, P2_READY_PATH, p2_pos, PLAYER_SCALE),
            p3_ready: init_sprite(s_size, id, P3_READY_PATH, p3_pos, PLAYER_SCALE),
            p4_ready: init_sprite(s_size, id, P4_READY_PATH, p4_pos, PLAYER_SCALE),

            // ========================= colored player cards =========================
            p1_me: init_sprite(s_size, id, P1_ME_PATH, p1_pos, PLAYER_SCALE),
            p2_me: init_sprite(s_size, id, P2_ME_PATH, p2_pos, PLAYER_SCALE),
            p3_me: init_sprite(s_size, id, P3_ME_PATH, p3_pos, PLAYER_SCALE),
            p4_me: init_sprite(s_size, id, P4_ME_PATH, p4_pos, PLAYER_SCALE),

            // ====================== colored ready player cards ======================
            p1_ready_me: init_sprite(s_size, id, P1_READY_ME_PATH, p1_pos, PLAYER_SCALE),
            p2_ready_me: init_sprite(s_size, id, P2_READY_ME_PATH, p2_pos, PLAYER_SCALE),
            p3_ready_me: init_sprite(s_size, id, P3_READY_ME_PATH, p3_pos, PLAYER_SCALE),
            p4_ready_me: init_sprite(s_size, id, P4_READY_ME_PATH, p4_pos, PLAYER_SCALE),

            // ================================ HUD ===================================
            crosshair: init_sprite(s_size, id, CROSSHAIR_PATH, bg_pos, CROSSHAIR_SCALE),

            p1_healthbar: init_with_anchor(s_size, id, P1_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
            p2_healthbar: init_with_anchor(s_size, id, P2_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
            p3_healthbar: init_with_anchor(s_size, id, P3_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
            p4_healthbar: init_with_anchor(s_size, id, P4_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),

            p1_halfbar: init_with_anchor(s_size, id, P1_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
            p2_halfbar: init_with_anchor(s_size, id, P2_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
            p3_halfbar: init_with_anchor(s_size, id, P3_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
            p4_halfbar: init_with_anchor(s_size, id, P4_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),

            p1_emptybar: init_with_anchor(s_size, id, P1_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
            p2_emptybar: init_with_anchor(s_size, id, P2_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
            p3_emptybar: init_with_anchor(s_size, id, P3_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
            p4_emptybar: init_with_anchor(s_size, id, P4_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
        
            ammo_0: init_with_anchor(s_size, id, AMMO_0_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_1: init_with_anchor(s_size, id, AMMO_1_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_2: init_with_anchor(s_size, id, AMMO_2_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_3: init_with_anchor(s_size, id, AMMO_3_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_4: init_with_anchor(s_size, id, AMMO_4_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_5: init_with_anchor(s_size, id, AMMO_5_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
            ammo_6: init_with_anchor(s_size, id, AMMO_6_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE), 

            p1_alive: init_sprite(s_size, id, P1_ALIVE_PATH, c1_pos, PLAYER_CIRCLE_SCALE),
            p2_alive: init_sprite(s_size, id, P2_ALIVE_PATH, c2_pos, PLAYER_CIRCLE_SCALE),
            p3_alive: init_sprite(s_size, id, P3_ALIVE_PATH, c3_pos, PLAYER_CIRCLE_SCALE),
            p4_alive: init_sprite(s_size, id, P4_ALIVE_PATH, c4_pos, PLAYER_CIRCLE_SCALE),

            p1_dead: init_sprite(s_size, id, P1_DEAD_PATH, c1_pos, PLAYER_CIRCLE_SCALE),
            p2_dead: init_sprite(s_size, id, P2_DEAD_PATH, c2_pos, PLAYER_CIRCLE_SCALE),
            p3_dead: init_sprite(s_size, id, P3_DEAD_PATH, c3_pos, PLAYER_CIRCLE_SCALE),
            p4_dead: init_sprite(s_size, id, P4_DEAD_PATH, c4_pos, PLAYER_CIRCLE_SCALE),

            p1_kill_p2: init_sprite(s_size, id, P1_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p1_kill_p3: init_sprite(s_size, id, P1_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p1_kill_p4: init_sprite(s_size, id, P1_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p2_kill_p1: init_sprite(s_size, id, P2_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p2_kill_p3: init_sprite(s_size, id, P2_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p2_kill_p4: init_sprite(s_size, id, P2_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p3_kill_p1: init_sprite(s_size, id, P3_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p3_kill_p2: init_sprite(s_size, id, P3_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p3_kill_p4: init_sprite(s_size, id, P3_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p4_kill_p1: init_sprite(s_size, id, P4_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p4_kill_p2: init_sprite(s_size, id, P4_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
            p4_kill_p3: init_sprite(s_size, id, P4_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE),
          
            damage: Fadable::new(init_sprite(s_size, id, DAMAGE_PATH, bg_pos, LOBBY_BG_SCALE), 1.0, 1.0),
            hitmarker: Fadable::new(init_sprite(s_size, id, HITMARKER_PATH, bg_pos, CROSSHAIR_SCALE), 3.0, 2.0),

            // =========================== game over elements ===========================
            game_over_bg: init_sprite(s_size, id, GAME_OVER_BG_PATH, bg_pos, LOBBY_BG_SCALE),
            winner_txt: init_sprite(s_size, id, WINNER_TXT_PATH, winner_txt_pos, WINNER_SCALE),
            continue_txt: init_sprite(s_size, id, CONTINUE_TXT_PATH, continue_pos, CONTINUE_SCALE),
            p1_winner: init_sprite(s_size, id, P1_JOINED_PATH, winner_pos, WINNER_SCALE),
            p2_winner: init_sprite(s_size, id, P2_JOINED_PATH, winner_pos, WINNER_SCALE),
            p3_winner: init_sprite(s_size, id, P3_JOINED_PATH, winner_pos, WINNER_SCALE),
            p4_winner: init_sprite(s_size, id, P4_JOINED_PATH, winner_pos, WINNER_SCALE),

            bar_header: init_sprite(s_size, id, BAR_HEADER_PATH, bar_header_pos, LEADERBOARD_SCALE),
            bar_1: init_sprite(s_size, id, BAR_1_PATH, bar_1_pos, LEADERBOARD_SCALE),
            bar_2: init_sprite(s_size, id, BAR_2_PATH, bar_2_pos, LEADERBOARD_SCALE),
            bar_3: init_sprite(s_size, id, BAR_3_PATH, bar_3_pos, LEADERBOARD_SCALE),
            bar_4: init_sprite(s_size, id, BAR_4_PATH, bar_4_pos, LEADERBOARD_SCALE),

            hits_0_b1: init_sprite(s_size, id, HITS_0_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_1_b1: init_sprite(s_size, id, HITS_1_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_2_b1: init_sprite(s_size, id, HITS_2_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_3_b1: init_sprite(s_size, id, HITS_3_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_4_b1: init_sprite(s_size, id, HITS_4_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_5_b1: init_sprite(s_size, id, HITS_5_PATH, bar_1_pos, LEADERBOARD_SCALE),
            hits_6_b1: init_sprite(s_size, id, HITS_6_PATH, bar_1_pos, LEADERBOARD_SCALE),

            hits_0_b2: init_sprite(s_size, id, HITS_0_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_1_b2: init_sprite(s_size, id, HITS_1_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_2_b2: init_sprite(s_size, id, HITS_2_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_3_b2: init_sprite(s_size, id, HITS_3_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_4_b2: init_sprite(s_size, id, HITS_4_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_5_b2: init_sprite(s_size, id, HITS_5_PATH, bar_2_pos, LEADERBOARD_SCALE),
            hits_6_b2: init_sprite(s_size, id, HITS_6_PATH, bar_2_pos, LEADERBOARD_SCALE),

            hits_0_b3: init_sprite(s_size, id, HITS_0_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_1_b3: init_sprite(s_size, id, HITS_1_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_2_b3: init_sprite(s_size, id, HITS_2_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_3_b3: init_sprite(s_size, id, HITS_3_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_4_b3: init_sprite(s_size, id, HITS_4_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_5_b3: init_sprite(s_size, id, HITS_5_PATH, bar_3_pos, LEADERBOARD_SCALE),
            hits_6_b3: init_sprite(s_size, id, HITS_6_PATH, bar_3_pos, LEADERBOARD_SCALE),

            hits_0_b4: init_sprite(s_size, id, HITS_0_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_1_b4: init_sprite(s_size, id, HITS_1_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_2_b4: init_sprite(s_size, id, HITS_2_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_3_b4: init_sprite(s_size, id, HITS_3_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_4_b4: init_sprite(s_size, id, HITS_4_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_5_b4: init_sprite(s_size, id, HITS_5_PATH, bar_4_pos, LEADERBOARD_SCALE),
            hits_6_b4: init_sprite(s_size, id, HITS_6_PATH, bar_4_pos, LEADERBOARD_SCALE),

            p1_text_b1: init_sprite(s_size, id, P1_TEXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            p2_text_b1: init_sprite(s_size, id, P2_TEXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            p3_text_b1: init_sprite(s_size, id, P3_TEXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            p4_text_b1: init_sprite(s_size, id, P4_TEXT_PATH, bar_1_pos, LEADERBOARD_SCALE),

            p1_text_b2: init_sprite(s_size, id, P1_TEXT_PATH, bar_2_pos, LEADERBOARD_SCALE),
            p2_text_b2: init_sprite(s_size, id, P2_TEXT_PATH, bar_2_pos, LEADERBOARD_SCALE),
            p3_text_b2: init_sprite(s_size, id, P3_TEXT_PATH, bar_2_pos, LEADERBOARD_SCALE),
            p4_text_b2: init_sprite(s_size, id, P4_TEXT_PATH, bar_2_pos, LEADERBOARD_SCALE),

            p1_text_b3: init_sprite(s_size, id, P1_TEXT_PATH, bar_3_pos, LEADERBOARD_SCALE),
            p2_text_b3: init_sprite(s_size, id, P2_TEXT_PATH, bar_3_pos, LEADERBOARD_SCALE),
            p3_text_b3: init_sprite(s_size, id, P3_TEXT_PATH, bar_3_pos, LEADERBOARD_SCALE),
            p4_text_b3: init_sprite(s_size, id, P4_TEXT_PATH, bar_3_pos, LEADERBOARD_SCALE),

            p1_text_b4: init_sprite(s_size, id, P1_TEXT_PATH, bar_4_pos, LEADERBOARD_SCALE),
            p2_text_b4: init_sprite(s_size, id, P2_TEXT_PATH, bar_4_pos, LEADERBOARD_SCALE),
            p3_text_b4: init_sprite(s_size, id, P3_TEXT_PATH, bar_4_pos, LEADERBOARD_SCALE),
            p4_text_b4: init_sprite(s_size, id, P4_TEXT_PATH, bar_4_pos, LEADERBOARD_SCALE),

            p1_you_text: init_sprite(s_size, id, P1_YOU_TEXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            p2_you_text: init_sprite(s_size, id, P2_YOU_TEXT_PATH, bar_2_pos, LEADERBOARD_SCALE),
            p3_you_text: init_sprite(s_size, id, P3_YOU_TEXT_PATH, bar_3_pos, LEADERBOARD_SCALE),
            p4_you_text: init_sprite(s_size, id, P4_YOU_TEXT_PATH, bar_4_pos, LEADERBOARD_SCALE),
        }
    }

    pub fn draw_game(&mut self, client_id: usize, client_alive: bool, client_ammo: u8, c_ecs: &Option<ClientECS>) {
        unsafe {
            self.crosshair.draw();
            self.hitmarker.draw();

            match client_ammo {
                0 => self.ammo_0.draw(),
                1 => self.ammo_1.draw(),
                2 => self.ammo_2.draw(),
                3 => self.ammo_3.draw(),
                4 => self.ammo_4.draw(),
                5 => self.ammo_5.draw(),
                6 => self.ammo_6.draw(),
                _ => ()
            }

            match c_ecs {
                Some(ecs) => {
                    for (i, player) in ecs.players.iter().enumerate() {
                        if ecs.health_components[*player].alive {
                            match i {
                                0 => self.p1_alive.draw(),
                                1 => self.p2_alive.draw(),
                                2 => self.p3_alive.draw(),
                                3 => self.p4_alive.draw(),
                                _ => ()
                            }
                        } else {
                            match i {
                                0 => self.p1_dead.draw(),
                                1 => self.p2_dead.draw(),
                                2 => self.p3_dead.draw(),
                                3 => self.p4_dead.draw(),
                                _ => ()
                            }
                        }

                        if i == client_id {
                            if client_alive && ecs.health_components[*player].health == 2 {
                                    match client_id {
                                    0 => self.p1_healthbar.draw(),
                                    1 => self.p2_healthbar.draw(),
                                    2 => self.p3_healthbar.draw(),
                                    3 => self.p4_healthbar.draw(),
                                    _ => ()
                                }
                            } else if client_alive && ecs.health_components[*player].health == 1 {
                                match client_id {
                                    0 => self.p1_halfbar.draw(),
                                    1 => self.p2_halfbar.draw(),
                                    2 => self.p3_halfbar.draw(),
                                    3 => self.p4_halfbar.draw(),
                                    _ => ()
                                }
                            } else {
                                match client_id {
                                    0 => self.p1_emptybar.draw(),
                                    1 => self.p2_emptybar.draw(),
                                    2 => self.p3_emptybar.draw(),
                                    3 => self.p4_emptybar.draw(),
                                    _ => ()
                                }
                            }
                        }
                    }
                }, 
                None => ()
            }
            self.damage.draw();

            self.p1_kill_p4.draw();
        }
    }

    pub fn draw_lobby(&mut self, l: &mut LobbyECS, client_id: usize) {
        unsafe {
            match client_id {
                0 => self.p1_lobby.draw(),
                1 => self.p2_lobby.draw(),
                2 => self.p3_lobby.draw(),
                3 => self.p4_lobby.draw(),
                _ => ()
            }

            match l.players.len() {
                0 => {
                    self.p1.draw();
                    self.p2.draw();
                    self.p3.draw();
                    self.p4.draw();
                },
                1 => {
                    if l.ready_players.contains_key(l.players[0]) {
                        if client_id == 0 { self.p1_ready_me.draw(); }
                        else { self.p1_ready.draw(); }
                    }
                    else {
                        if client_id == 0 { self.p1_me.draw(); }
                        else { self.p1_joined.draw(); }
                    }

                    self.p2.draw();
                    self.p3.draw();
                    self.p4.draw();
                },
                2 => {
                    if l.ready_players.contains_key(l.players[0]) {
                        if client_id == 0 { self.p1_ready_me.draw(); }
                        else { self.p1_ready.draw(); }
                    }
                    else {
                        if client_id == 0 { self.p1_me.draw(); }
                        else { self.p1_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[1]) {
                        if client_id == 1 { self.p2_ready_me.draw(); }
                        else { self.p2_ready.draw(); }
                    }
                    else {
                        if client_id == 1 { self.p2_me.draw(); }
                        else { self.p2_joined.draw(); }
                    }

                    self.p3.draw();
                    self.p4.draw();
                },
                3 => {
                    if l.ready_players.contains_key(l.players[0]) {
                        if client_id == 0 { self.p1_ready_me.draw(); }
                        else { self.p1_ready.draw(); }
                    }
                    else {
                        if client_id == 0 { self.p1_me.draw(); }
                        else { self.p1_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[1]) {
                        if client_id == 1 { self.p2_ready_me.draw(); }
                        else { self.p2_ready.draw(); }
                    }
                    else {
                        if client_id == 1 { self.p2_me.draw(); }
                        else { self.p2_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[2]) {
                        if client_id == 2 { self.p3_ready_me.draw(); }
                        else { self.p3_ready.draw(); }
                    }
                    else {
                        if client_id == 2 { self.p3_me.draw(); }
                        else { self.p3_joined.draw(); }
                    }

                    self.p4.draw();
                },
                4 => {
                    if l.ready_players.contains_key(l.players[0]) {
                        if client_id == 0 { self.p1_ready_me.draw(); }
                        else { self.p1_ready.draw(); }
                    }
                    else {
                        if client_id == 0 { self.p1_me.draw(); }
                        else { self.p1_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[1]) {
                        if client_id == 1 { self.p2_ready_me.draw(); }
                        else { self.p2_ready.draw(); }
                    }
                    else {
                        if client_id == 1 { self.p2_me.draw(); }
                        else { self.p2_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[2]) {
                        if client_id == 2 { self.p3_ready_me.draw(); }
                        else { self.p3_ready.draw(); }
                    }
                    else {
                        if client_id == 2 { self.p3_me.draw(); }
                        else { self.p3_joined.draw(); }
                    }

                    if l.ready_players.contains_key(l.players[3]) {
                        if client_id == 3 { self.p4_ready_me.draw(); }
                        else { self.p4_ready.draw(); }
                    }
                    else {
                        if client_id == 3 { self.p4_me.draw(); }
                        else { self.p4_joined.draw(); }
                    }
                },
                _ => ()
            }
        }
    }

    pub fn draw_game_over(&mut self, curr_id: usize, c_ecs: &Option<ClientECS>, rankings: &mut Vec<usize>) {
        unsafe{
            self.game_over_bg.draw();
            self.winner_txt.draw();
            self.continue_txt.draw();
            self.bar_header.draw();
            self.bar_1.draw();
            self.bar_2.draw();
            self.bar_3.draw();
            self.bar_4.draw();

            match c_ecs {
                Some(ecs) => {
                    for (i, player) in ecs.players.iter().enumerate() {
                        if  ecs.players.contains(player) &&
                            ecs.health_components[*player].alive &&
                            ecs.health_components[*player].health > 0
                        {
                            match i {
                                0 => {
                                    self.p1_winner.draw();
                                    self.p1_text_b1.draw();
                                }
                                1 => {
                                    self.p2_winner.draw();
                                    self.p2_text_b1.draw();
                                }
                                2 => {
                                    self.p3_winner.draw();
                                    self.p3_text_b1.draw();
                                }
                                3 => {
                                    self.p4_winner.draw();
                                    self.p4_text_b1.draw();
                                }
                                _ => ()
                            }
                        }
                    }

                    for (i, player) in rankings.iter().enumerate() {
                        if i == 0 {             // row 2
                            match player {
                                0 => self.p1_text_b2.draw(),
                                1 => self.p2_text_b2.draw(),
                                2 => self.p3_text_b2.draw(),
                                3 => self.p4_text_b2.draw(),
                                _ => ()
                            }
                        } else if i == 1 {      // row 3
                            match player {
                                0 => self.p1_text_b3.draw(),
                                1 => self.p2_text_b3.draw(),
                                2 => self.p3_text_b3.draw(),
                                3 => self.p4_text_b3.draw(),
                                _ => ()
                            }
                        } else if i == 2 {      // row 4
                            match player {
                                0 => self.p1_text_b4.draw(),
                                1 => self.p2_text_b4.draw(),
                                2 => self.p3_text_b4.draw(),
                                3 => self.p4_text_b4.draw(),
                                _ => ()
                            }
                        }
                    }
                }
                None => ()
            }

            self.hits_3_b1.draw();
            self.hits_2_b2.draw();
            self.hits_1_b3.draw();
            self.hits_0_b4.draw();

            // self.hits_9_b1.set_position(vec2(0.0, 0.0));
            // self.hits_9_b1.draw();
            // self.hits_9_b1.set_position(vec2(100.0, 100.0));
            // self.hits_9_b1.draw();
        }
    }

    pub fn draw_splash(&mut self) {
        unsafe { self.splash.draw() };
    }
}

fn init_sprite(s_size: Vector2<f32>, shader_id: u32, path: &str, 
    position: Vector2<f32>, percentage: f32) -> Sprite
{
    unsafe {
        let mut sprite = Sprite::new(s_size, shader_id);
        sprite.set_texture(path);
        sprite.set_position(position);
        sprite.set_percentage_width(s_size, percentage);
        sprite
    }
}

fn init_with_anchor(s_size: Vector2<f32>, shader_id: u32, path: &str, 
    position: Vector2<f32>, anchor: Anchor, percentage: f32) -> Sprite
{
    unsafe {
        let mut sprite = Sprite::new(s_size, shader_id);
        sprite.set_texture(path);
        sprite.set_position(position);
        sprite.set_anchor(anchor);
        sprite.set_percentage_width(s_size, percentage);
        sprite
    }
}