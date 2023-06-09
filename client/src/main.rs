
mod macros; // this needs to stay at the top
mod audio;

mod camera;
mod common;
mod lasso;
mod mesh;
mod model;
mod shader;
mod skybox;
mod sprite_renderer;
mod tracker;
mod ui;
mod util;
mod screenshake;
mod fadable;

mod force_field;
mod init_skies;
mod init_models;
mod velocity_indicator;
mod arm;
mod tracer;

use std::collections::HashMap;
use std::time::Duration;

// graphics
extern crate gl;
extern crate glfw;

use self::glfw::{Action, Context, Key};
use cgmath::InnerSpace;
use cgmath::{
    perspective, vec2, vec3, vec4, Deg, EuclideanSpace, Matrix4, Point3, Quaternion,
    Transform, Vector3, Vector4
};

use std::ffi::CStr;

use crate::camera::*;
use crate::model::Model;
use crate::shader::Shader;
use crate::audio::AudioPlayer;
use crate::common::*;
use crate::force_field::ForceField;
use crate::lasso::Lasso;
use crate::tracker::Tracker;
use crate::velocity_indicator::VelocityIndicator;
use crate::arm::Arm;
use crate::tracer::TracerManager;

// network
use shared::shared_components::*;
use shared::shared_functions::*;
use shared::*;
use std::io::{self, Read};
use std::net::{ToSocketAddrs, TcpStream};
use std::process;

use slotmap::{SecondaryMap,DefaultKey};

type Entity = DefaultKey;

#[derive(PartialEq, Eq)]
enum GameState {
    EnteringLobby,
    InLobby,
    InGame,
    GameOver
}

fn main() -> io::Result<()> {
    // initialize event map for handled events
    let mut client_events: SecondaryMap<Entity, ()> = SecondaryMap::new();

    // initialize audio manager
    let mut audio = AudioPlayer::new();
  
    // create camera and camera information
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut first_mouse = true;
    let mut first_click = false;
    let mut first_enter = false;
    let mut last_x: f32;
    let mut last_y: f32;
    let mut fullscreen = false;
    let mut f11_pressed = false;

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
    let mut refresh_rate = 0;
    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            width = glfw::Monitor::get_physical_size(m.expect("access monitor for width")).0 as u32;
            height = glfw::Monitor::get_physical_size(m.expect("access monitor for height")).1 as u32;
            refresh_rate = glfw::Monitor::get_video_mode(m.expect("access monitor for video mode")).expect("failed to get video mode").refresh_rate;
            glfw.create_window(
                width * 2,
                height * 2,
                WINDOW_TITLE,
                glfw::WindowMode::Windowed,
            )
        })
        .expect("Failed to create GLFW window.");

    let mut saved_xpos = window.get_pos().0;
    let mut saved_ypos = window.get_pos().1;
    let mut saved_width = width;
    let mut saved_height = height;
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

    // Set up OpenGL shaders
    let (shader_program, sprite_shader) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // create shaders
        let shader_program = Shader::new("shaders/light.vs", "shaders/light.fs");
        let sprite_shader = Shader::new("shaders/sprite.vs", "shaders/sprite.fs");

        // actually allow transparency
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        (shader_program, sprite_shader)
    };

    // set up ui
    let mut ui_elems = ui::UI::initialize(screen_size, sprite_shader.id, width as f32, height as f32);
    let mut rankings = Vec::new();;

    // render splash screen
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    ui_elems.draw_splash();
    window.swap_buffers();

    // textures for skybox
    let skies = init_skies::init_skyboxes();
    let mut sky: usize = 0;

    // add all models to hashmap
    let models: HashMap<String, Model> = init_models::init_models();

    // set up tracker
    let tracker_colors: [Vector4<f32>; 4] = [
        vec4(224.0 / 255.0, 14.0 / 255.0, 115.0 / 255.0, 1.0),
        vec4(98.0 / 255.0, 168.0 / 255.0, 205.0 / 255.0, 1.0),
        vec4(252.0 / 255.0, 201.0 / 255.0, 0.0 / 255.0, 1.0),
        vec4(88.0 / 255.0, 180.0 / 255.0, 36.0 / 255.0, 1.0),
    ];
    let mut tracker = unsafe {
        let tracker = Tracker::new(sprite_shader.id, 0.9, vec2(width as f32, height as f32));
        tracker
    };

    // set up tracers
    let mut tracers = TracerManager::new(vec![
        Model::new("resources/tracer/tracer_p1.obj"),
        Model::new("resources/tracer/tracer_p2.obj"),
        Model::new("resources/tracer/tracer_p3.obj"),
        Model::new("resources/tracer/tracer_p4.obj"),
    ], screen_size);

    // create force field
    let force_field = ForceField::new(250.0, screen_size);

    // create lasso
    let lasso = Lasso::new();

    // create velocity indicator
    let mut vel_indicator = VelocityIndicator::new();

    // create first person model
    let mut arm = Arm::new();

    // client ECS to be sent to server
    let mut client_ecs: Option<ClientECS> = None;

    // lobby ECS to player updates in lobby
    let mut lobby_ecs = LobbyECS::new();

    // set up loop variables
    let mut game_state = GameState::InLobby;
    let mut is_focused = true;
    let mut ready_sent = false;
    let mut spectator_mode = false;

    // Create network TcpStream
    // TODO: change to connect_timeout?
    let mut stream = loop {
        let addrs = (SERVER_ADDR.to_string() + ":" + &PORT.to_string()).to_socket_addrs().expect("Error loading socket address");
        match TcpStream::connect_timeout(&addrs.last().unwrap(), Duration::from_millis(TICK_SPEED)) {
            Ok(s) => break s,
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                process_events_lobby(&events);
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
                ui_elems.draw_splash();
                window.swap_buffers();
                glfw.poll_events();
                //toggle fullscreen
                if !f11_pressed && window.get_key(Key::F11) == Action::Press {
                    fullscreen = !fullscreen;
                    set_fullscreen(fullscreen, &mut glfw, &mut window, &mut width, &mut height, &mut saved_xpos, &mut saved_ypos, &mut saved_width, &mut saved_height, refresh_rate);
                    f11_pressed = true;
                }
                if window.get_key(Key::F11) == Action::Release {
                    f11_pressed = false;
                }
            },
            Err(e) => panic!("Error connecting to server: {}",e)
        }
    };

    // receive and save client id
    let mut read_buf = [0u8, 1];
    stream.read(&mut read_buf).unwrap();
    let client_id = read_buf[0] as usize;
    println!("client id: {}", client_id);

    stream
        .set_nonblocking(true)
        .expect("Failed to set stream as nonblocking");

    let mut curr_id = client_id;

    let mut frame_count = 0;
    let mut delta_time;
    let mut last_frame = 0.0;

    let mut vel_prev: Vector3<f32> = vec3(0.0,0.0,0.0);

    // WINDOW LOOP
    // -----------
    loop {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // set cursor mode based on is_focused
        if is_focused && game_state != GameState::InLobby {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }

        match game_state {
            GameState::EnteringLobby => {
                rankings.clear();
                ready_sent = false; // prevents sending ready message twice
                game_state = GameState::InLobby;
            }
            GameState::InLobby => {
                process_inputs_lobby(
                    &mut window,
                    &mut ready_sent,
                    &mut first_enter,
                    &mut stream
                );

                process_events_lobby(&events);

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    if lobby_ecs.ids.len() > 0 {
                        curr_id = lobby_ecs.players.iter().position(|&r| r == lobby_ecs.ids[client_id]).unwrap();
                        gl::DepthMask(gl::FALSE);
                        ui_elems.draw_lobby(&mut lobby_ecs, curr_id);
                        gl::DepthMask(gl::TRUE);
                    }
                }

                // poll server for ready message or ready-player updates
                let received = read_data(&mut stream);
                if received.len() > 0 {
                    // ignore malformed input (probably leftover game state)
                    let res: Result<LobbyECS, bitcode::Error> = bitcode::deserialize(&received);
                    match res {
                        Ok(l_ecs) => {
                            lobby_ecs = l_ecs.clone();

                            if lobby_ecs.start_game {
                                sky = lobby_ecs.sky;
                                let start_pos = &lobby_ecs.position_components[lobby_ecs.ids[client_id]];
                                camera.RotQuat = Quaternion::new(start_pos.qw, start_pos.qx, start_pos.qy, start_pos.qz);
                                camera.UpdateVecs();
                                client_ecs = None;
                                first_mouse = true;
                                lobby_ecs.ready_players.clear();
                                game_state = GameState::InGame;
                            }
                        }
                        _ => (),
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

                let mut player_vel = vec3(0.0, 0.0, 0.0);

                process_inputs_game(
                    &mut window,
                    &mut input_component,
                    &mut roll,
                    &mut first_click,
                    is_focused,
                );

                process_events_game(
                    &events,
                    &mut first_mouse,
                    &mut last_x,
                    &mut last_y,
                    &mut camera,
                    roll,
                    is_focused,
                );

                // set camera front of input_component
                input_component.camera_qx = camera.RotQuat.v.x;
                input_component.camera_qy = camera.RotQuat.v.y;
                input_component.camera_qz = camera.RotQuat.v.z;
                input_component.camera_qw = camera.RotQuat.s;

                // send client data if player is still alive
                if client_health.alive {
                    let j = bitcode::serialize(&input_component)
                        .expect("Input component serialization error");
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
                            eprintln!("Failed to read message size from server: {}", e);
                            process::exit(1);
                        }
                    }
                    let s_size = size.try_into().unwrap();
                    let mut read_buf = vec![0 as u8; s_size];
                    match stream.peek(&mut read_buf) {
                        Ok(bytes_read) if bytes_read == s_size => {
                            // if this throws an error we deserve to crash tbh
                            stream
                                .read_exact(&mut read_buf)
                                .expect("read_exact did not read the same amount of bytes as peek");
                            // let message: &str = str::from_utf8(&read_buf[4..])
                            //     .expect("Error converting buffer to string");
                            // // TODO: handle this throwing an error. Occasionally crashes ^
                            let value: ClientECS = bitcode::deserialize(&read_buf[4..])
                                .expect("Error converting string to ClientECS");
                            client_ecs = Some(value);
                        }
                        Ok(_) => {
                            break;
                        }
                        Err(e) => {
                            eprintln!("Failed to read message from server: {}", e);
                            process::exit(1);
                        }
                    }
                }

                match &client_ecs {
                    Some(c_ecs) => {
                        let player_key = c_ecs.ids[client_id];
                        let mut screenshake_event = false;
                        // make sure we haven't handled this event yet
                        for &event in &c_ecs.events {
                            if client_events.contains_key(event) {
                                continue;
                            }
                            client_events.insert(event, ());
                            // check event type
                            // skip audio events for all but client 0 if we're debugging on same machine
                            if c_ecs.audio_components.contains_key(event) && (!AUDIO_DEBUG || client_id == 0) {
                                let audio_event = &c_ecs.audio_components[event];
                                match &mut audio {
                                    Some(audioplayer) => {
                                        match audioplayer.play_sound(&audio_event.name, audio_event.x, audio_event.y, audio_event.z){
                                            Ok(_) => (),
                                            Err(e) => eprintln!("Audio error playing sound: {e}"),
                                        };
                                    },
                                    None => ()
                                }
                            }
                            match c_ecs.event_components[event].event_type {
                                EventType::FireEvent { player } => {
                                    if player == player_key {
                                        camera.ScreenShake.add_trauma(0.3);
                                        arm.shoot();
                                        screenshake_event = true;
                                    }
                                },
                                EventType::HitEvent { player, target , hit_x, hit_y, hit_z} => {
                                    if target == player_key && c_ecs.health_components[player_key].alive {
                                        camera.ScreenShake.add_trauma(0.5);
                                        screenshake_event = true;
                                        ui_elems.damage.add_alpha(0.6);
                                    } else if player == player_key && c_ecs.players.contains(&target) && c_ecs.health_components[target].alive {
                                        ui_elems.hitmarker.add_alpha(1.0);
                                    }
                                    let player_id = c_ecs.players.iter().position(|&x| x == player).unwrap();
                                    tracers.add_tracer(player_id, &c_ecs.position_components[player], vec3(hit_x, hit_y, hit_z), player == player_key);
                                },
                                EventType::ReloadEvent { player } => {
                                    if player == player_key {
                                        arm.reload();
                                    }
                                },
                                EventType::DeathEvent { player, killer } => {
                                    let k_id = c_ecs.players.iter().position(|&x| x == killer).unwrap();
                                    let p_id = c_ecs.players.iter().position(|&x| x == player).unwrap();

                                    ui_elems.display_death_message(k_id, p_id);
                                    
                                    rankings.push(c_ecs.players.iter().position(|&x| x == player).unwrap());
                                    if player == player_key {
                                        camera.ScreenShake.add_trauma(1.0);
                                        ui_elems.damage.add_alpha(1.0);
                                    } else if killer == player_key {
                                        let target_id = c_ecs.players.iter().position(|&x| x == player).unwrap();
                                        ui_elems.killmarkers[target_id % ui_elems.killmarkers.len()].add_alpha(2.0);
                                    }
                                }, 
                                EventType::DisconnectEvent { player } => {
                                    println!("a disconnect happened");
                                    rankings.push(c_ecs.players.iter().position(|&x| x == player).unwrap());
                                }
                            }
                        }

                        // player velocity
                        let velocity = &c_ecs.velocity_components[player_key];
                        player_vel = vec3(velocity.vel_x, velocity.vel_y, velocity.vel_z);
                        if !screenshake_event {
                            // kinetic energy should be more realistic, but feels wrong
                            // let delta_ke = (0.5 * velocity.mass * (player_vel.magnitude().powi(2) - vel_prev.magnitude().powi(2))).abs();
                            // if delta_ke > 0.0 {
                            //     camera.ScreenShake.add_trauma(delta_ke / 1000.0);
                            //     println!("KE change: {}", delta_ke);
                            // }

                            // change in velocity feels better
                            let delta_v = (player_vel - vel_prev).magnitude();
                            let delta_speed = (player_vel.magnitude() - vel_prev.magnitude()).abs();
                            if delta_speed > 0.0 {
                                camera.ScreenShake.add_trauma(delta_speed / 100.0);
                            }
                        }
                        vel_prev = player_vel;

                        // dead player camera
                        if !c_ecs.health_components[player_key].alive && !spectator_mode {
                            camera.RotQuat = Quaternion::new(
                                c_ecs.position_components[player_key].qw,
                                c_ecs.position_components[player_key].qx,
                                c_ecs.position_components[player_key].qy,
                                c_ecs.position_components[player_key].qz,
                            );
                            camera.UpdateVecs();
                        }
                    },
                    None => ()
                }

                camera.ScreenShake.shake_camera();

                // render
                // ------
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    // activate shader
                    shader_program.use_program();

                    // TODO: lighting variables (this can imported from a json file?)
                    shader_program.set_vector3(c_str!("lightDir"), &skies[sky].light_dir);
                    shader_program.set_vector3(c_str!("lightAmb"), &skies[sky].light_ambience);
                    shader_program.set_vector3(c_str!("lightDif"), &skies[sky].light_diffuse);

                    let mut trackers = vec![];

                    // NEEDS TO BE REWORKED FOR MENU STATE
                    match &client_ecs {
                        Some(c_ecs) => {
                            let player_key = c_ecs.ids[client_id];
                            client_ammo = c_ecs.weapon_components[player_key].ammo;

                            // handle changes in client health
                            if c_ecs.health_components[player_key].alive
                                && c_ecs.health_components[player_key].health
                                    != client_health.health
                            {
                                client_health.health = c_ecs.health_components[player_key].health;
                            } else if c_ecs.health_components[player_key].alive
                                != client_health.alive
                                && client_health.alive
                            {
                                client_health.alive = c_ecs.health_components[player_key].alive;
                            } else {
                                client_health.alive = c_ecs.health_components[player_key].alive;
                                client_health.health = c_ecs.health_components[player_key].health;
                            }

                            // setup player camera
                            let player_pos = vec3(
                                c_ecs.position_components[player_key].x,
                                c_ecs.position_components[player_key].y,
                                c_ecs.position_components[player_key].z,
                            );
                            match &mut audio {
                                Some(audioplayer) if frame_count == 0 => {
                                    match audioplayer.move_listener(player_pos.x, player_pos.y, player_pos.z, camera.RotQuat.v.x, camera.RotQuat.v.y, camera.RotQuat.v.z, camera.RotQuat.s) {
                                        Ok(_) => (),
                                        Err(e) => eprintln!("Audio error moving listener: {e}"),
                                    };
                                },
                                _ => ()
                            }

                            if !client_health.alive && input_component.enter_pressed {
                                spectator_mode = true;
                            }

                            if !client_health.alive && spectator_mode {
                                camera.ProcessKeyboard(&input_component, delta_time, &shader_program, width, height);
                                shader_program.set_vector3(c_str!("viewPos"), &camera.Position.to_vec());
                            } else {
                                set_camera_pos(&mut camera, player_pos, &shader_program, width, height);
                                shader_program.set_vector3(c_str!("viewPos"), &camera.Position.to_vec());
                            }

                            for &renderable in &c_ecs.renderables {
                                let model_name = &c_ecs.model_components[renderable].modelname;
                                if renderable == player_key && !spectator_mode {
                                    continue;
                                }
                                if !models.contains_key(model_name) {
                                    eprintln!("Models map does not contain key: {}", model_name);
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
                                let rot_mat = Matrix4::from(Quaternion::new(
                                    model_qw, model_qx, model_qy, model_qz,
                                ));

                                // setup scale matrix
                                let scale_mat =
                                    Matrix4::from_scale(c_ecs.model_components[renderable].scale);

                                let model = pos_mat * scale_mat * rot_mat;
                                shader_program.set_mat4(c_str!("model"), &model);
                                let model_scaleless = pos_mat * rot_mat;
                                shader_program.set_mat4(c_str!("model_scaleless"), &model_scaleless);
                                models[model_name].draw(&shader_program);
                            }

                            let mut i = 0;
                            for &player in &c_ecs.players {
                                // lasso
                                if c_ecs.player_lasso_components.contains_key(player) {
                                    // setup position matrix
                                    let player_x = c_ecs.position_components[player].x;
                                    let player_y = c_ecs.position_components[player].y;
                                    let player_z = c_ecs.position_components[player].z;
                                    let model_pos = vec3(player_x, player_y, player_z);
                                    let pos_mat = Matrix4::from_translation(model_pos);

                                    // setup rotation matrix
                                    let player_qx = c_ecs.position_components[player].qx;
                                    let player_qy = c_ecs.position_components[player].qy;
                                    let player_qz = c_ecs.position_components[player].qz;
                                    let player_qw = c_ecs.position_components[player].qw;
                                    let rot_mat = Matrix4::from(Quaternion::new(
                                        player_qw, player_qx, player_qy, player_qz,
                                    ));

                                    // setup scale matrix
                                    let scale_mat =
                                        Matrix4::from_scale(c_ecs.model_components[player].scale);

                                    let model = pos_mat * scale_mat * rot_mat;

                                    let anchor_x = c_ecs.player_lasso_components[player].anchor_x;
                                    let anchor_y = c_ecs.player_lasso_components[player].anchor_y;
                                    let anchor_z = c_ecs.player_lasso_components[player].anchor_z;

                                    // draw lasso
                                    let lasso_origin = if player == player_key {
                                        Point3::new(-0.5, 0.0, 0.0)
                                    } else {
                                        Point3::new(-0.5, -0.4, 0.0)
                                    };
                                    let lasso_p1 = model.transform_point(lasso_origin);
                                    let lasso_p2 = vec3(anchor_x, anchor_y, anchor_z);
                                    lasso.draw_btw_points(
                                        lasso_p1.to_vec(),
                                        lasso_p2,
                                        &shader_program,
                                    );
                                }

                                // draw trackers
                                if player != player_key && c_ecs.health_components[player].alive {
                                    let pos = &c_ecs.position_components[player];
                                    let pos = vec3(pos.x, pos.y, pos.z);
                                    tracker.draw_tracker(
                                        &camera,
                                        pos,
                                        tracker_colors[i % tracker_colors.len()],
                                        &mut trackers,
                                    );
                                }
                                i += 1;
                            }

                            // game has ended
                            if c_ecs.game_ended {
                                for (i, player) in c_ecs.players.iter().enumerate() {
                                    if  c_ecs.players.contains(player) &&
                                        c_ecs.health_components[*player].alive &&
                                        c_ecs.health_components[*player].health > 0
                                    {
                                        rankings.push(i);
                                    }
                                }
                                rankings.reverse();
                                game_state = GameState::GameOver;
                            }
                        }
                        None => set_camera_pos(
                            &mut camera,
                            vec3(0.0, 0.0, 0.0),
                            &shader_program,
                            width,
                            height,
                        ),
                    }
                    // note: the first iteration through the match{} above draws the model without view and projection setup

                    // draw skybox
                    let projection: Matrix4<f32> = perspective(
                        Deg(camera.Zoom),
                        width as f32 / height as f32,
                        0.1,
                        100.0
                    );

                    skies[sky].skybox.draw(camera.GetViewMatrix(), projection);

                    // enable translucency for force field
                    gl::DepthMask(gl::FALSE);

                    force_field.draw(&camera, camera.Position.to_vec());
                    tracers.draw_tracers(&camera);

                    // disable translucency for velocity indicator and first person model
                    gl::DepthMask(gl::TRUE);
                    // HUD elements should always be rendered on top
                    if !spectator_mode {
                        gl::Clear(gl::DEPTH_BUFFER_BIT);

                        arm.draw(&camera, &shader_program);
                        vel_indicator.draw(&camera, player_vel, width as f32 / height as f32, &shader_program);
                    }

                    // enable translucency for 2D HUD
                    gl::DepthMask(gl::FALSE);

                    tracker.draw_all_trackers(trackers);
                    ui_elems.draw_game(curr_id, client_health.alive, client_ammo, &client_ecs, spectator_mode);

                    // disable translucency for next loop
                    gl::DepthMask(gl::TRUE);

                    frame_count += 1;
                    frame_count %= AUDIO_FRAMES;
                }
            }
            GameState::GameOver => {
                spectator_mode = false;

                unsafe{
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    process_events_game_over(&events);

                    if process_inputs_game_over(&mut window, &mut first_enter) {
                        game_state = GameState::EnteringLobby;
                    }
                    gl::DepthMask(gl::FALSE);
                    ui_elems.draw_game_over(curr_id, &client_ecs, &mut rankings);
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

        //toggle fullscreen
        if !f11_pressed && window.get_key(Key::F11) == Action::Press {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen, &mut glfw, &mut window, &mut width, &mut height, &mut saved_xpos, &mut saved_ypos, &mut saved_width, &mut saved_height, refresh_rate);
            f11_pressed = true;
        }
        if window.get_key(Key::F11) == Action::Release {
            f11_pressed = false;
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}