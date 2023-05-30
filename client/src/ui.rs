use crate::sprite_renderer::{Anchor, Sprite};
use cgmath::{vec2, Array, Vector2};
use shared::*;
use shared::shared_components::*;

pub struct UI {
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
    pub p4_dead: Sprite
}

impl UI {
    pub fn initialize(s_size: Vector2<f32>, shader_id: u32, width: f32, height: f32) -> UI {
        UI {

            // ===== lobby background =====

            p1_lobby: init_sprite(
                s_size, shader_id, LOBBY_BG_1_PATH,
                vec2(width / 2.0, height / 2.0),
                LOBBY_BG_SCALE
            ),

            p2_lobby: init_sprite(
                s_size, shader_id, LOBBY_BG_2_PATH,
                vec2(width / 2.0, height / 2.0),
                LOBBY_BG_SCALE
            ),

            p3_lobby: init_sprite(
                s_size, shader_id, LOBBY_BG_3_PATH,
                vec2(width / 2.0, height / 2.0),
                LOBBY_BG_SCALE
            ),

            p4_lobby: init_sprite(
                s_size, shader_id, LOBBY_BG_4_PATH,
                vec2(width / 2.0, height / 2.0),
                LOBBY_BG_SCALE
            ),
            
            // ===== gray player cards ======

            p1: init_sprite(
                s_size, shader_id, P1_PATH,
                vec2(width / 5.0, height / 2.0),
                PLAYER_SCALE
            ),

            p2: init_sprite(
                s_size, shader_id, P2_PATH,
                vec2(width / 2.5, height / 2.0),
                PLAYER_SCALE
            ),

            p3: init_sprite(
                s_size, shader_id, P3_PATH,
                vec2(width / 1.67, height / 2.0),
                PLAYER_SCALE
            ),

            p4: init_sprite(
                s_size, shader_id, P4_PATH,
                vec2(width / 1.25, height / 2.0),
                PLAYER_SCALE
            ),

            // ===== client joined player cards =====

            p1_joined: init_sprite(
                s_size, shader_id, P1_JOINED_PATH,
                vec2(width / 5.0, height / 2.0),
                PLAYER_SCALE
            ),

            p2_joined: init_sprite(
                s_size, shader_id, P2_JOINED_PATH,
                vec2(width / 2.5, height / 2.0),
                PLAYER_SCALE
            ),

            p3_joined: init_sprite(
                s_size, shader_id, P3_JOINED_PATH,
                vec2(width / 1.67, height / 2.0),
                PLAYER_SCALE
            ),

            p4_joined: init_sprite(
                s_size, shader_id, P4_JOINED_PATH,
                vec2(width / 1.25, height / 2.0),
                PLAYER_SCALE
            ),

            // ===== ready player cards =====

            p1_ready: init_sprite(
                s_size, shader_id, P1_READY_PATH,
                vec2(width / 5.0, height / 2.0),
                PLAYER_SCALE
            ),

            p2_ready: init_sprite(
                s_size, shader_id, P2_READY_PATH,
                vec2(width / 2.5, height / 2.0),
                PLAYER_SCALE
            ),

            p3_ready: init_sprite(
                s_size, shader_id, P3_READY_PATH,
                vec2(width / 1.67, height / 2.0),
                PLAYER_SCALE
            ),

            p4_ready: init_sprite(
                s_size, shader_id, P4_READY_PATH,
                vec2(width / 1.25, height / 2.0),
                PLAYER_SCALE
            ),

            // ===== colored player cards =====

            p1_me: init_sprite(
                s_size, shader_id, P1_ME_PATH,
                vec2(width / 5.0, height / 2.0),
                PLAYER_SCALE
            ),

            p2_me: init_sprite(
                s_size, shader_id, P2_ME_PATH,
                vec2(width / 2.5, height / 2.0),
                PLAYER_SCALE
            ),

            p3_me: init_sprite(
                s_size, shader_id, P3_ME_PATH,
                vec2(width / 1.67, height / 2.0),
                PLAYER_SCALE
            ),

            p4_me: init_sprite(
                s_size, shader_id, P4_ME_PATH,
                vec2(width / 1.25, height / 2.0),
                PLAYER_SCALE
            ),

            // ===== colored ready player cards =====

            p1_ready_me: init_sprite(
                s_size, shader_id, P1_READY_ME_PATH,
                vec2(width / 5.0, height / 2.0),
                PLAYER_SCALE
            ),

            p2_ready_me: init_sprite(
                s_size, shader_id, P2_READY_ME_PATH,
                vec2(width / 2.5, height / 2.0),
                PLAYER_SCALE
            ),

            p3_ready_me: init_sprite(
                s_size, shader_id, P3_READY_ME_PATH,
                vec2(width / 1.67, height / 2.0),
                PLAYER_SCALE
            ),

            p4_ready_me: init_sprite(
                s_size, shader_id, P4_READY_ME_PATH,
                vec2(width / 1.25, height / 2.0),
                PLAYER_SCALE
            ),

            // ===== HUD =====

            crosshair: init_sprite(
                s_size, shader_id, CROSSHAIR_PATH,
                vec2(width / 2.0, height / 2.0),
                CROSSHAIR_SCALE
            ),

            p1_healthbar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p1-health-full.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p2_healthbar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p2-health-full.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p3_healthbar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p3-health-full.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p4_healthbar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p4-health-full.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p1_emptybar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p1-health-empty.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p2_emptybar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p2-health-empty.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p3_emptybar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p3-health-empty.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),

            p4_emptybar: init_with_anchor(
                s_size, shader_id,
                "resources/ui_textures/health_bar/p4-health-empty.png",
                vec2(5.0, 5.0), 
                Anchor::BotLeft, BAR_SCALE
            ),
        
            ammo_0: init_with_anchor(
                s_size, shader_id, AMMO_0_PATH,
                vec2(width - 5.0, 10.0),
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_1: init_with_anchor(
                s_size, shader_id, AMMO_1_PATH,
                vec2(width - 5.0, 10.0),
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_2: init_with_anchor(
                s_size, shader_id, AMMO_2_PATH,
                vec2(width - 5.0, 10.0), 
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_3: init_with_anchor(
                s_size, shader_id, AMMO_3_PATH,
                vec2(width - 5.0, 10.0), 
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_4: init_with_anchor(
                s_size, shader_id, AMMO_4_PATH,
                vec2(width - 5.0, 10.0), 
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_5: init_with_anchor(
                s_size, shader_id, AMMO_5_PATH,
                vec2(width - 5.0, 10.0), 
                Anchor::BotRight, BAR_SCALE
            ),
        
            ammo_6: init_with_anchor(
                s_size, shader_id, AMMO_6_PATH,
                vec2(width - 5.0, 10.0), 
                Anchor::BotRight, BAR_SCALE
            ), 

            p1_alive: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p1-circle.png",
                vec2(width / 5.0, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p2_alive: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p2-circle.png",
                vec2(width / 2.5, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p3_alive: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p3-circle.png",
                vec2(width / 1.67, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p4_alive: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p4-circle.png",
                vec2(width / 1.25, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p1_dead: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p1-circle-gray.png",
                vec2(width / 5.0, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p2_dead: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p2-circle-gray.png",
                vec2(width / 2.5, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p3_dead: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p3-circle-gray.png",
                vec2(width / 1.67, height / 2.0),
                PLAYER_CIRCLE_SCALE
            ),

            p4_dead: init_sprite(
                s_size, shader_id,
                "resources/ui_textures/player_circles/p4-circle-gray.png",
                vec2(width / 1.25, height / 2.0),
                PLAYER_CIRCLE_SCALE
            )
        }
    }

    pub fn draw_game(&mut self, client_id: usize, client_alive: bool, client_ammo: u8, c_ecs: &Option<ClientECS>) {
        unsafe {
            self.crosshair.draw();

            if client_alive {
                match client_id {
                    0 => self.p1_healthbar.draw(),
                    1 => self.p2_healthbar.draw(),
                    2 => self.p3_healthbar.draw(),
                    3 => self.p4_healthbar.draw(),
                    _ => ()
                }
            }
            else {
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
        // TODO: optimize this !!!!

        unsafe {
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
            
            match client_id {
                0 => { self.p1_lobby.draw(); },
                1 => { self.p2_lobby.draw(); },
                2 => { self.p3_lobby.draw(); },
                3 => { self.p4_lobby.draw(); },
                _ => ()
            }
        }
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