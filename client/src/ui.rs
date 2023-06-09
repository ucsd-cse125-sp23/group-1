use crate::fadable::Fadable;
use crate::sprite_renderer::{Anchor, Sprite};
use cgmath::{vec2, Vector2, vec3};
use shared::*;
use shared::shared_components::*;

pub struct UI {
    // ========================== splash ui elements ==========================
    splash: Sprite,

    // ========================== lobby ui elements ===========================
    lobby_bg: [Sprite; 4],
    player_card_gray: [Sprite; 4],
    player_card_joined: [Sprite; 4],
    player_card_ready: [Sprite; 4],
    player_card_joined_me: [Sprite; 4],
    player_card_ready_me: [Sprite; 4],

    // =========================== game ui elements ===========================
    crosshair: Sprite,

    health_bar_full: [Sprite; 4],
    health_bar_half: [Sprite; 4],
    health_bar_empty: [Sprite; 4],

    ammo: [Sprite; 7],
    //ammo_reload: [Sprite; 12],

    player_circle_alive: [Sprite; 4],
    player_circle_dead: [Sprite; 4],

    death_messages: [[Option<Fadable>; 4]; 4],

    pub damage: Fadable,
    pub hitmarker: Fadable,

    // ======================== game over ui elements =========================
    game_over_bg: Sprite,
    winner_txt: Sprite,
    continue_txt: Sprite,

    winner_card: [Sprite; 4],
    
    bar_header: Sprite,
    leaderboard_bar: [Sprite; 4],
    
    hits: [Sprite; 7],
    player_txt: [Sprite; 4],
    player_you_txt: [Sprite; 4],

    bar_pos: [Vector2<f32>; 4]
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

        let death_message_fade = 0.3;
        let death_message_alpha = 3.0;

        UI {
            // ============================ splash screen =============================
            splash: init_sprite(s_size, id, SPLASH_PATH, bg_pos, LOBBY_BG_SCALE),

            // =========================== lobby background ===========================
            lobby_bg: [
                init_sprite(s_size, id, LOBBY_BG_1_PATH, bg_pos, LOBBY_BG_SCALE),
                init_sprite(s_size, id, LOBBY_BG_2_PATH, bg_pos, LOBBY_BG_SCALE),
                init_sprite(s_size, id, LOBBY_BG_3_PATH, bg_pos, LOBBY_BG_SCALE),
                init_sprite(s_size, id, LOBBY_BG_4_PATH, bg_pos, LOBBY_BG_SCALE),
            ],
            
            // =========================== gray player cards ==========================
            player_card_gray: [
                init_sprite(s_size, id, P1_PATH, p1_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P2_PATH, p2_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P3_PATH, p3_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P4_PATH, p4_pos, PLAYER_SCALE),
            ],

            // ====================== client joined player cards ======================
            player_card_joined: [
                init_sprite(s_size, id, P1_JOINED_PATH, p1_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P2_JOINED_PATH, p2_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P3_JOINED_PATH, p3_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P4_JOINED_PATH, p4_pos, PLAYER_SCALE),
            ],

            // ========================== ready player cards ==========================
            player_card_ready: [
                init_sprite(s_size, id, P1_READY_PATH, p1_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P2_READY_PATH, p2_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P3_READY_PATH, p3_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P4_READY_PATH, p4_pos, PLAYER_SCALE),
            ],

            // ========================= colored player cards (me) =========================
            player_card_joined_me: [
                init_sprite(s_size, id, P1_ME_PATH, p1_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P2_ME_PATH, p2_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P3_ME_PATH, p3_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P4_ME_PATH, p4_pos, PLAYER_SCALE),
            ],

            // ====================== colored ready player cards (me) ======================
            player_card_ready_me: [
                init_sprite(s_size, id, P1_READY_ME_PATH, p1_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P2_READY_ME_PATH, p2_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P3_READY_ME_PATH, p3_pos, PLAYER_SCALE),
                init_sprite(s_size, id, P4_READY_ME_PATH, p4_pos, PLAYER_SCALE),
            ],

            // ================================ HUD ===================================
            crosshair: init_sprite(s_size, id, CROSSHAIR_PATH, bg_pos, CROSSHAIR_SCALE),

            health_bar_full: [
                init_with_anchor(s_size, id, P1_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P2_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P3_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P4_HEALTH_FULL, health_pos, Anchor::BotLeft, BAR_SCALE),
            ],

            health_bar_half: [
                init_with_anchor(s_size, id, P1_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P2_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P3_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P4_HEALTH_HALF, health_pos, Anchor::BotLeft, BAR_SCALE),
            ],

            health_bar_empty: [
                init_with_anchor(s_size, id, P1_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P2_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P3_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
                init_with_anchor(s_size, id, P4_HEALTH_EMPTY, health_pos, Anchor::BotLeft, BAR_SCALE),
            ],

            ammo: [
                init_with_anchor(s_size, id, AMMO_0_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_1_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_2_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_3_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_4_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_5_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE),
                init_with_anchor(s_size, id, AMMO_6_PATH, ammo_pos, Anchor::BotRight, BAR_SCALE), 
            ],

            player_circle_alive: [
                init_sprite(s_size, id, P1_ALIVE_PATH, c1_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P2_ALIVE_PATH, c2_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P3_ALIVE_PATH, c3_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P4_ALIVE_PATH, c4_pos, PLAYER_CIRCLE_SCALE),
            ],

            player_circle_dead: [
                init_sprite(s_size, id, P1_DEAD_PATH, c1_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P2_DEAD_PATH, c2_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P3_DEAD_PATH, c3_pos, PLAYER_CIRCLE_SCALE),
                init_sprite(s_size, id, P4_DEAD_PATH, c4_pos, PLAYER_CIRCLE_SCALE),
            ],

            death_messages: [
                [
                    None,
                    Some(Fadable::new(init_sprite(s_size, id, P1_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P1_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P1_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha))
                ],
                [
                    Some(Fadable::new(init_sprite(s_size, id, P2_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    None,
                    Some(Fadable::new(init_sprite(s_size, id, P2_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P2_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha))
                ],
                [
                    Some(Fadable::new(init_sprite(s_size, id, P3_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P3_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    None,
                    Some(Fadable::new(init_sprite(s_size, id, P3_KILL_P4_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha))
                ],
                [
                    Some(Fadable::new(init_sprite(s_size, id, P4_KILL_P1_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P4_KILL_P2_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    Some(Fadable::new(init_sprite(s_size, id, P4_KILL_P3_PATH, death_message_pos, DEATH_MESSAGE_SCALE), death_message_fade, death_message_alpha)),
                    None
                ]
            ],
          
            damage: Fadable::new(init_sprite(s_size, id, DAMAGE_PATH, bg_pos, LOBBY_BG_SCALE), 1.0, 1.0),
            hitmarker: Fadable::new(init_sprite(s_size, id, HITMARKER_PATH, bg_pos, CROSSHAIR_SCALE), 3.0, 2.0),

            // =========================== game over elements ===========================
            game_over_bg: init_sprite(s_size, id, GAME_OVER_BG_PATH, bg_pos, LOBBY_BG_SCALE),
            winner_txt: init_sprite(s_size, id, WINNER_TXT_PATH, winner_txt_pos, WINNER_SCALE),
            continue_txt: init_sprite(s_size, id, CONTINUE_TXT_PATH, continue_pos, CONTINUE_SCALE),

            winner_card: [
                init_sprite(s_size, id, P1_JOINED_PATH, winner_pos, WINNER_SCALE),
                init_sprite(s_size, id, P2_JOINED_PATH, winner_pos, WINNER_SCALE),
                init_sprite(s_size, id, P3_JOINED_PATH, winner_pos, WINNER_SCALE),
                init_sprite(s_size, id, P4_JOINED_PATH, winner_pos, WINNER_SCALE),
            ],

            bar_header: init_sprite(s_size, id, BAR_HEADER_PATH, bar_header_pos, LEADERBOARD_SCALE),
            leaderboard_bar: [
                init_sprite(s_size, id, BAR_1_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, BAR_2_PATH, bar_2_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, BAR_3_PATH, bar_3_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, BAR_4_PATH, bar_4_pos, LEADERBOARD_SCALE),
            ],

            hits: [
                init_sprite(s_size, id, HITS_0_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_1_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_2_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_3_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_4_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_5_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, HITS_6_PATH, bar_1_pos, LEADERBOARD_SCALE),
            ],

            player_txt: [
                init_sprite(s_size, id, P1_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P2_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P3_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P4_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            ],

            player_you_txt: [
                init_sprite(s_size, id, P1_YOU_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P2_YOU_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P3_YOU_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
                init_sprite(s_size, id, P4_YOU_TXT_PATH, bar_1_pos, LEADERBOARD_SCALE),
            ],

            bar_pos: [
                bar_1_pos,
                bar_2_pos,
                bar_3_pos,
                bar_4_pos,
            ]
        }
    }

    pub fn draw_game(&mut self, client_id: usize, client_alive: bool, client_ammo: u8, c_ecs: &Option<ClientECS>) {
        unsafe {
            self.crosshair.draw();
            self.hitmarker.draw();
            self.ammo[client_ammo as usize].draw();

            match c_ecs {
                Some(ecs) => {
                    for (i, player) in ecs.players.iter().enumerate() {
                        if ecs.health_components[*player].alive {
                            self.player_circle_alive[i].draw();
                        } else {
                            self.player_circle_dead[i].draw();
                        }

                        if i == client_id {
                            if client_alive && ecs.health_components[*player].health == 2 {
                                self.health_bar_full[client_id].draw();
                            } else if client_alive && ecs.health_components[*player].health == 1 {
                                self.health_bar_half[client_id].draw();
                            } else {
                                self.health_bar_empty[client_id].draw();
                            }
                        }
                    }
                }, 
                None => ()
            }

            let mut num_deathmessages = 0.0;

            for row in &mut self.death_messages {
                for deathmessage in row {
                    match deathmessage {
                        Some(message) => {
                            let pos = message.sprite.transform.position;
                            message.sprite.set_position(pos + vec2(0.0, -20.0 * num_deathmessages));
                            message.draw();
                            message.sprite.set_position(pos);
                            if message.alpha > 0.0 {
                                num_deathmessages += 1.0;
                            }
                        },
                        None => ()
                    }
                }
            }

            self.damage.draw();
        }
    }

    pub fn draw_lobby(&mut self, l: &mut LobbyECS, curr_id: usize) {
        unsafe {
            self.lobby_bg[curr_id].draw();

            for i in 0..4 {
                if i >= l.players.len() {
                    self.player_card_gray[i].draw();
                }
                else if l.ready_players.contains_key(l.players[i]) {
                    if curr_id == i { self.player_card_ready_me[i].draw(); }
                    else { self.player_card_ready[i].draw(); }
                }
                else {
                    if curr_id == i { self.player_card_joined_me[i].draw(); }
                    else { self.player_card_joined[i].draw(); }
                }
            }
        }
    }

    pub fn draw_game_over(&mut self, curr_id: usize, c_ecs: &Option<ClientECS>, rankings: &mut Vec<usize>) {
        unsafe{
            self.game_over_bg.draw();
            self.winner_txt.draw();
            self.continue_txt.draw();
            self.bar_header.draw();

            for i in 0..4 {
                self.leaderboard_bar[i].draw();
            }

            match c_ecs {
                Some(ecs) => {
                    for (i, player) in rankings.iter().enumerate() {
                        if i == 0 {
                            self.winner_card[*player].draw();
                        }

                        if curr_id == *player {
                            self.player_you_txt[*player].set_position(self.bar_pos[i]);
                            self.player_you_txt[*player].draw(); 
                        } else {
                            self.player_txt[*player].set_position(self.bar_pos[i]);
                            self.player_txt[*player].draw(); 
                        }
                        
                        let hit_count = &mut self.hits[ecs.health_components[ecs.players[*player]].hits as usize];
                        hit_count.set_position(self.bar_pos[i]);
                        hit_count.draw();
                    }
                }
                None => ()
            }
        }
    }

    pub fn draw_splash(&mut self) {
        unsafe { self.splash.draw() };
    }

    pub fn display_death_message(&mut self, killer: usize, player: usize) {
        println!("player {} shot player {}", killer, player);
        match &mut self.death_messages[killer][player] {
            Some(message) => message.add_alpha(2.0),
            None => eprintln!("Player {killer} shot themselves (Player {player})!")
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