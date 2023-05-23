use crate::sprite_renderer::{Anchor, Sprite};
use cgmath::{vec2, Array, Vector2};
use shared::*;
use shared::shared_components::*;

pub struct UI {
    pub lobby: Sprite,
    // pub p1: Sprite,
    // pub p2: Sprite,
    // pub p3: Sprite,
    // pub p4: Sprite,
    // pub p1_joined: Sprite,
    // pub p2_joined: Sprite,
    // pub p3_joined: Sprite,
    // pub p4_joined: Sprite,
    // pub p1_ready: Sprite,
    // pub p2_ready: Sprite,
    // pub p3_ready: Sprite,
    // pub p4_ready: Sprite,

    pub crosshair: Sprite,
    pub full_healthbar: Sprite,
    pub empty_healthbar: Sprite,
    pub ammo_0: Sprite,
    pub ammo_1: Sprite,
    pub ammo_2: Sprite,
    pub ammo_3: Sprite,
    pub ammo_4: Sprite,
    pub ammo_5: Sprite,
    pub ammo_6: Sprite,
}

impl UI {
    pub fn initialize(screen_size: Vector2<f32>, shader_id: u32, width: f32, height: f32) -> UI {
        UI {
            lobby: init_sprite(
                screen_size, shader_id,
                "resources/ui_textures/lobby_background.png",
                vec2(width / 2.0, height / 2.0),
                LOBBY_BG_SCALE
            ),

            // p1: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p1.png",
            //     vec2(width / 5.0, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p2: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p2.png",
            //     vec2(width / 2.5, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p3: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p3.png",
            //     vec2(width / 1.67, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p4: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p4.png",
            //     vec2(width / 1.25, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p1_joined: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p1_joined.png",
            //     vec2(width / 5.0, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p2_joined: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p2_joined.png",
            //     vec2(width / 2.5, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p3_joined: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p3_joined.png",
            //     vec2(width / 1.67, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p4_joined: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p4_joined.png",
            //     vec2(width / 1.25, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p1_ready: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p1_ready.png",
            //     vec2(width / 5.0, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p2_ready: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p2_ready.png",
            //     vec2(width / 2.5, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p3_ready: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p3_ready.png",
            //     vec2(width / 1.67, height / 2.0),
            //     PLAYER_SCALE
            // ),

            // p4_ready: init_sprite(
            //     screen_size, shader_id,
            //     "resources/ui_textures/p4_ready.png",
            //     vec2(width / 1.25, height / 2.0),
            //     PLAYER_SCALE
            // ),

            crosshair: init_sprite(
                screen_size, shader_id,
                "resources/ui_textures/crosshair.png",
                vec2(width / 2.0, height / 2.0),
                CROSSHAIR_SCALE
            ),

            empty_healthbar: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/emptyHealthBar.png",
                vec2(5.0, height - 5.0), 
                Anchor::TopLeft, BAR_SCALE
            ),
        
            full_healthbar: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/fullHealthBar.png",
                vec2(5.0, height - 5.0), 
                Anchor::TopLeft, BAR_SCALE
            ),
        
            ammo_0: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo0.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_1: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo1.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_2: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo2.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_3: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo3.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_4: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo4.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_5: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo5.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            ),
        
            ammo_6: init_sprite_with_anchor(
                screen_size, shader_id,
                "resources/ui_textures/ammo6.png",
                vec2(width - 5.0, height - 5.0), 
                Anchor::TopRight, BAR_SCALE
            )
        }
    }

    pub fn draw_game(&mut self, client_alive: bool, client_ammo: u8) {
        unsafe {
            self.crosshair.draw();

            if client_alive { self.full_healthbar.draw(); }
            else { self.empty_healthbar.draw(); }

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
        }
    }

    pub fn draw_lobby(&mut self, l: &mut LobbyECS) {
        unsafe {
            // match l.players.len() {
            //     0 => {
            //         self.p1.draw();
            //         self.p2.draw();
            //         self.p3.draw();
            //         self.p4.draw();
            //     },
            //     1 => {
            //         if l.ready_players.contains_key(l.players[0]){ self.p1_ready.draw(); }
            //         else { self.p1_joined.draw(); }
            //         self.p2.draw();
            //         self.p3.draw();
            //         self.p4.draw();
            //     },
            //     2 => {
            //         if l.ready_players.contains_key(l.players[0]){ self.p1_ready.draw(); }
            //         else { self.p1_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[1]){ self.p2_ready.draw(); }
            //         else { self.p2_joined.draw(); }
            //         self.p3.draw();
            //         self.p4.draw();
            //     },
            //     3 => {
            //         if l.ready_players.contains_key(l.players[0]){ self.p1_ready.draw(); }
            //         else { self.p1_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[1]){ self.p2_ready.draw(); }
            //         else { self.p2_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[2]){ self.p3_ready.draw(); }
            //         else { self.p3_joined.draw(); }
            //         self.p4.draw();
            //     },
            //     4 => {
            //         if l.ready_players.contains_key(l.players[0]){ self.p1_ready.draw(); }
            //         else { self.p1_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[1]){ self.p2_ready.draw(); }
            //         else { self.p2_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[2]){ self.p3_ready.draw(); }
            //         else { self.p3_joined.draw(); }
            //         if l.ready_players.contains_key(l.players[3]){ self.p4_ready.draw(); }
            //         else { self.p4_joined.draw(); }
            //     },
            //     _ => ()
            // }
            self.lobby.draw();
        }
    }
}

fn init_sprite(screen_size: Vector2<f32>, shader_id: u32, path: &str, 
    position: Vector2<f32>, scale: f32) -> Sprite
{
    unsafe {
        let mut sprite = Sprite::new(screen_size, shader_id);
        sprite.set_texture(path);
        sprite.set_position(position);
        sprite.set_scale(Vector2::from_value(scale));
        sprite
    }
}

fn init_sprite_with_anchor(screen_size: Vector2<f32>, shader_id: u32, path: &str, 
    position: Vector2<f32>, anchor: Anchor, scale: f32) -> Sprite
{
    unsafe {
        let mut sprite = Sprite::new(screen_size, shader_id);
        sprite.set_texture(path);
        sprite.set_position(position);
        sprite.set_anchor(anchor);
        sprite.set_scale(Vector2::from_value(scale));
        sprite
    }
}