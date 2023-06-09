use crate::fadable::Fadable;
use crate::sprite_renderer::{Anchor, Sprite};
use cgmath::{vec2, Vector2};
use shared::*;
use shared::shared_components::*;

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
    pub damage: Fadable,
    pub hitmarker: Fadable,
    pub killmarkers: [Fadable; 4],

    // ======================== game over ui elements =========================
    pub game_over_bg: Sprite,
    pub winner_txt: Sprite,
    pub p1_winner: Sprite,
    pub p2_winner: Sprite,
    pub p3_winner: Sprite,
    pub p4_winner: Sprite
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

        let winner_pos = vec2(width / 4.00, height / 2.4);
        let winner_txt_pos = vec2(width / 4.00, height / 1.16);

        let health_pos = vec2(BAR_BORDER, BAR_BORDER);
        let ammo_pos = vec2(width - BAR_BORDER, AMMO_BAR_BORDER);

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
          
            damage: Fadable::new(init_sprite(s_size, id, DAMAGE_PATH, bg_pos, LOBBY_BG_SCALE), 1.0, 1.0),
            hitmarker: Fadable::new(init_sprite(s_size, id, HITMARKER_PATH, bg_pos, HITMARKER_SCALE), 3.0, 2.0),

            killmarkers: [
                Fadable::new(init_sprite(s_size, id, P1_KILLMARKER_PATH, bg_pos, HITMARKER_SCALE), 1.0, 2.0),
                Fadable::new(init_sprite(s_size, id, P2_KILLMARKER_PATH, bg_pos, HITMARKER_SCALE), 1.0, 2.0),
                Fadable::new(init_sprite(s_size, id, P3_KILLMARKER_PATH, bg_pos, HITMARKER_SCALE), 1.0, 2.0),
                Fadable::new(init_sprite(s_size, id, P4_KILLMARKER_PATH, bg_pos, HITMARKER_SCALE), 1.0, 2.0),
            ],

            // =========================== game over elements ===========================
            game_over_bg: init_sprite(s_size, id, GAME_OVER_BG_PATH, bg_pos, LOBBY_BG_SCALE),
            winner_txt: init_sprite(s_size, id, WINNER_TXT_PATH, winner_txt_pos, WINNER_SCALE),
            p1_winner: init_sprite(s_size, id, P1_JOINED_PATH, winner_pos, WINNER_SCALE),
            p2_winner: init_sprite(s_size, id, P2_JOINED_PATH, winner_pos, WINNER_SCALE),
            p3_winner: init_sprite(s_size, id, P3_JOINED_PATH, winner_pos, WINNER_SCALE),
            p4_winner: init_sprite(s_size, id, P4_JOINED_PATH, winner_pos, WINNER_SCALE)
        }
    }

    pub fn draw_game(&mut self, client_id: usize, client_alive: bool, client_ammo: u8, c_ecs: &Option<ClientECS>, spectator_mode: bool) {
        unsafe {
            if !spectator_mode {
                self.crosshair.draw();
                for killmarker in &mut self.killmarkers {
                    killmarker.draw();
                }
                self.hitmarker.draw();
              

                if client_alive {
                    match client_id {
                        0 => self.p1_healthbar.draw(),
                        1 => self.p2_healthbar.draw(),
                        2 => self.p3_healthbar.draw(),
                        3 => self.p4_healthbar.draw(),
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

                self.damage.draw();
            }

            // draw player status regardless of spectator mode or not
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
                    }
                }, 
                None => ()
            }
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

    pub fn draw_game_over(&mut self, c_ecs: &Option<ClientECS>) {
        unsafe{
            self.game_over_bg.draw();
            match c_ecs {
                Some(ecs) => {
                    for (i, player) in ecs.players.iter().enumerate() {
                        if  ecs.players.contains(player) &&
                            ecs.health_components[*player].alive &&
                            ecs.health_components[*player].health > 0
                        {
                            match i {
                                0 => self.p1_winner.draw(),
                                1 => self.p2_winner.draw(),
                                2 => self.p3_winner.draw(),
                                3 => self.p4_winner.draw(),
                                _ => ()
                            }
                        }
                    }
                }
                None => ()
            }
            self.winner_txt.draw();
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