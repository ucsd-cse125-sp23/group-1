mod camera;
mod macros;
mod mesh;
mod model;
mod shader;
mod skybox;
mod sprite_renderer;
mod util;
mod tracker;
mod common;
mod ui;

use std::collections::HashMap;

// graphics
extern crate gl;
extern crate glfw;

use self::glfw::{Context, Key, Action};
use cgmath::{Matrix4, Quaternion, Deg, vec3, perspective, Point3, vec4, vec2, Vector4};

use std::ffi::{CStr};

use crate::camera::*;
use crate::model::Model;
use crate::shader::Shader;
use crate::skybox::Skybox;

// network
use std::io::{self, Read};
use std::net::{TcpStream};
use std::process;
use std::str;
use shared::shared_components::*;
use shared::shared_functions::*;
use shared::*;
use crate::common::*;
use crate::tracker::Tracker;

enum GameState {
    EnteringLobby,
    InLobby,
    InGame,
}

fn main() -> io::Result<()> {
    // create camera and camera information
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut first_mouse = true;
    let mut first_click = false;
    let mut last_x: f32; let mut last_y: f32;

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let mut width = 0;
    let mut height = 0;
    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            width = glfw::Monitor::get_physical_size(m.expect("access monitor for width")).0 as u32;
            height = glfw::Monitor::get_physical_size(m.expect("access monitor for width")).1 as u32;
            glfw.create_window(
                width * 2,
                height * 2,
                WINDOW_TITLE,
                glfw::WindowMode::Windowed,
            )
        })
        .expect("Failed to create GLFW window.");

    last_x = width as f32 / 2.0;
    last_y = height as f32 / 2.0;
    let screen_size = vec2(width as f32, height as f32);

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_close_polling(true);
    window.set_aspect_ratio(width, height);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    // Create network TcpStream
    // TODO: change to connect_timeout?
    let mut stream =
        TcpStream::connect(SERVER_ADDR.to_string() + ":" + &PORT.to_string())?;

    // receive and save client id
    let mut read_buf = [0u8, 1];
    stream.read(&mut read_buf).unwrap();
    let client_id = read_buf[0] as usize;
    println!("client id: {}", client_id);

    stream
        .set_nonblocking(true)
        .expect("Failed to set stream as nonblocking");

    // Set up OpenGL shaders
    let (shader_program, sprite_shader) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // create shaders
        let shader_program = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        let sprite_shader = Shader::new("shaders/sprite.vs", "shaders/sprite.fs");

        // actually allow transparency
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        (shader_program, sprite_shader)
    };

    // textures for skybox
    let skybox = unsafe{ Skybox::new("resources/skybox/space", ".png") };

    // add all models to hashmap
    let mut models: HashMap<String, Model> = HashMap::new();
    models.insert("cube".to_string(), Model::new("resources/cube/cube.obj"));

    // set up tracker
    let tracker_colors: [Vector4<f32>; 3] = [
        vec4(0.91797, 0.25, 0.2031, 1.0),
        vec4(0.2031, 0.7852, 0.91797, 1.0),
        vec4(0.3867, 0.9648, 0.0781, 1.0),
    ];
    let mut tracker = unsafe {
        let tracker = Tracker::new(sprite_shader.id, 1.0, vec2(width as f32, height as f32));
        tracker
    };

    // set up ui
    let mut ui_elems = ui::UI::initialize(screen_size, sprite_shader.id, width as f32, height as f32);

    // client ECS to be sent to server
    let mut client_ecs: Option<ClientECS> = None;

    // lobby ECS to player updates in lobby
    let mut lobby_ecs = LobbyECS::new();

    // set up loop variables
    let mut game_state = GameState::InLobby;
    let mut is_focused = true;
    let mut ready_sent = false;

    // WINDOW LOOP
    // -----------
    loop {
        // set cursor mode based on is_focused
        if is_focused {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }

        match game_state {
            GameState::EnteringLobby => {
                ready_sent = false; // prevents sending ready message twice
                println!("Press ENTER when ready to start game");
                game_state = GameState::InLobby;
            }
            GameState::InLobby => {
                process_inputs_lobby(&mut window, &mut ready_sent, &mut stream);

                // events
                // ------
                process_events_lobby(&events);

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    // TOOD: lobby_ecs.ids[client_id] get index of element in lobby_ecs.players
                    ui_elems.draw_lobby(&mut lobby_ecs, client_id);
                }

                // poll server for ready message or ready-player updates
                let received = read_data(&mut stream);
                if received.len() > 0 {
                    // ignore malformed input (probably leftover game state)
                    let res : Result<LobbyECS, serde_json::Error> = serde_json::from_str(received.as_str());
                    match res {
                        Ok(l_ecs) => {
                            // TODO: faster way to copy l_ecs into lobby_ecs ???
                            lobby_ecs.name_components = l_ecs.name_components;
                            lobby_ecs.players = l_ecs.players;
                            lobby_ecs.ids = l_ecs.ids;
                            lobby_ecs.start_game = l_ecs.start_game;
                            lobby_ecs.position_components = l_ecs.position_components;
                            lobby_ecs.ready_players = l_ecs.ready_players;

                            if lobby_ecs.start_game {
                                println!("Game starting!");
                                let start_pos = &lobby_ecs.position_components[lobby_ecs.ids[client_id]];
                                camera.RotQuat = Quaternion::new(start_pos.qw, start_pos.qx, start_pos.qy, start_pos.qz);
                                camera.UpdateVecs();
                                client_ecs = None;
                                first_mouse = true;
                                game_state = GameState::InGame;
                            }
                        }
                        _ => ()
                    }
                }
            }
            GameState::InGame => {
                // initialize components and variables
                let mut client_ammo = 0;
                let mut client_health = PlayerHealthComponent::default();
                let mut input_component = PlayerInputComponent::default();
                let mut size_buf = [0 as u8; 4];

                let mut roll = false;

                // process inputs
                // --------------
                process_inputs_game(
                    &mut window,
                    &mut input_component,
                    &mut roll,
                    &mut first_click,
                    is_focused
                );

                // events
                // ------
                process_events_game(
                    &events,
                    &mut first_mouse,
                    &mut last_x,
                    &mut last_y,
                    &mut camera, 
                    roll,
                    is_focused
                );

                // set camera front of input_component
                input_component.camera_qx = camera.RotQuat.v.x;
                input_component.camera_qy = camera.RotQuat.v.y;
                input_component.camera_qz = camera.RotQuat.v.z;
                input_component.camera_qw = camera.RotQuat.s;

                // send client data if player is still alive
                if client_health.alive {
                    let j = serde_json::to_string(&input_component).expect("Input component serialization error");
                    write_data(&mut stream, j);
                } // TODO: support spectator movement

                // receive all incoming server data
                loop {
                    let size: u32;
                    match stream.peek(&mut size_buf) {
                        Ok(4) => {
                            // big-endian for networks. it's tradition, dammit!
                            size = u32::from_be_bytes(size_buf);
                        }
                        Ok(_) => {
                            // incomplete size field, wait for next tick
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            eprintln!("Failed to read message size from server: {}",e);
                            process::exit(1);
                        }
                    }
                    let s_size = size.try_into().unwrap();
                    let mut read_buf = vec![0 as u8; s_size];
                    match stream.peek(&mut read_buf) {
                        Ok(bytes_read) if bytes_read == s_size => {
                            // if this throws an error we deserve to crash tbh
                            stream.read_exact(&mut read_buf).expect("read_exact did not read the same amount of bytes as peek");
                            let message : &str = str::from_utf8(&read_buf[4..]).expect("Error converting buffer to string");
                            // TODO: handle this throwing an error. Occasionally crashes ^
                            let value : ClientECS = serde_json::from_str(message).expect("Error converting string to ClientECS");
                            client_ecs = Some(value);
                        }
                        Ok(_) => {
                            break;
                        }
                        Err(e) => {
                            eprintln!("Failed to read message from server: {}",e);
                            process::exit(1);
                        }
                    }
                }

                // render
                // ------
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    // activate shader
                    shader_program.use_program();

                    let mut trackers = vec![];

                    // NEEDS TO BE REWORKED FOR MENU STATE
                    match &client_ecs {
                        Some(c_ecs) => {
                            let player_key = c_ecs.ids[client_id];
                            client_ammo = c_ecs.weapon_components[player_key].ammo;

                            // handle changes in client health
                            if c_ecs.health_components[player_key].alive && c_ecs.health_components[player_key].health != client_health.health {
                                client_health.health = c_ecs.health_components[player_key].health;
                                println!("Player {} is still alive, with {} lives left", client_id, client_health.health);
                            } else if c_ecs.health_components[player_key].alive != client_health.alive && client_health.alive {
                                client_health.alive = c_ecs.health_components[player_key].alive;
                                println!("Player {} is no longer alive x_x", client_id);
                            } else {
                                client_health.alive = c_ecs.health_components[player_key].alive;
                                client_health.health = c_ecs.health_components[player_key].health;
                            }

                            let player_pos = vec3(
                                c_ecs.position_components[player_key].x,
                                c_ecs.position_components[player_key].y,
                                c_ecs.position_components[player_key].z
                            );
                            set_camera_pos(&mut camera, player_pos, &shader_program, width, height);

                            for &renderable in &c_ecs.renderables {
                                if renderable == player_key {
                                    continue;
                                }
                                
                                // setup position matrix
                                let model_x = c_ecs.position_components[renderable].x;
                                let model_y = c_ecs.position_components[renderable].y;
                                let model_z = c_ecs.position_components[renderable].z;
                                let model_pos = vec3(model_x, model_y, model_z);
                                let pos_mat = Matrix4::from_translation(model_pos);

                                // setup rotation matrix
                                let model_qx = c_ecs.position_components[renderable].qx;
                                let model_qy = c_ecs.position_components[renderable].qy;
                                let model_qz = c_ecs.position_components[renderable].qz;
                                let model_qw = c_ecs.position_components[renderable].qw;
                                let rot_mat = Matrix4::from(Quaternion::new(model_qw, model_qx, model_qy, model_qz));

                                // setup scale matrix (skip for now)
                                let scale_mat = Matrix4::from_scale(c_ecs.model_components[renderable].scale);
                            
                                let model = pos_mat * scale_mat * rot_mat;
                                shader_program.set_mat4(c_str!("model"), &model);
                                let model_name = &c_ecs.model_components[renderable].modelname;
                                models[model_name].draw(&shader_program);
                            }

                            // draw trackers
                            let mut i = 0;
                            for &player in &c_ecs.players {
                                if player != player_key && c_ecs.health_components[player].alive {
                                    let pos = &c_ecs.position_components[player];
                                    let pos = vec3(pos.x, pos.y, pos.z);
                                    tracker.draw_tracker(&camera, pos, tracker_colors[i%tracker_colors.len()], &mut trackers);
                                }
                                i += 1;
                            }

                            // game has ended
                            if c_ecs.game_ended {
                                for (i, player) in c_ecs.ids.iter().enumerate() {
                                    if  c_ecs.players.contains(player) && 
                                        c_ecs.health_components[*player].alive &&
                                        c_ecs.health_components[*player].health > 0 
                                    {
                                        println!("The winner is player {}!", i);
                                    }
                                }
                                game_state = GameState::EnteringLobby;
                            }
                        }
                        None => {
                            set_camera_pos(&mut camera, vec3(0.0,0.0,0.0), &shader_program, width, height)
                        }
                    }
                    // note: the first iteration through the match{} above draws the model without view and projection setup
                
                    // draw skybox
                    let projection: Matrix4<f32> = perspective(
                        Deg(camera.Zoom),
                        width as f32 / height as f32,
                        0.1,
                        100.0
                    );
                    skybox.draw(camera.GetViewMatrix(), projection);

                    ui_elems.draw_game(client_health.alive, client_ammo);

                    gl::DepthMask(gl::FALSE);
                    tracker.draw_all_trackers(trackers);
                    gl::DepthMask(gl::TRUE);
                }
            }
        }

        // change is_focused after key press
        if is_focused && window.get_key(Key::Escape) == Action::Press {
            is_focused = false;
        }
        // Refocus by clicking on window
        if !is_focused && window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
            is_focused = true;
            first_mouse = true;
            first_click = true;

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}