mod camera;
mod macros;
mod mesh;
mod model;
mod shader;
mod skybox;
mod sprite_renderer;
mod util;
mod lasso;
mod tracker;

use std::collections::HashMap;

// graphics
extern crate gl;
extern crate glfw;

use self::glfw::{Action, Context, Key, MouseButton};
use cgmath::{perspective, vec2, vec3, Deg, Matrix4, Point3, Quaternion, Vector3, Vector2, Array, EuclideanSpace, Transform, Vector4, vec4};

use std::ffi::{CStr};
use std::sync::mpsc::Receiver;

use crate::camera::*;
use crate::model::Model;
use crate::shader::Shader;
use crate::skybox::Skybox;
use crate::sprite_renderer::{Anchor, Sprite};

// network
use std::io::{self, Read};
use std::net::{TcpStream};
use std::process;
use std::str;
use shared::*;
use shared::shared_components::*;
use shared::shared_functions::*;
use crate::lasso::Lasso;
use crate::tracker::Tracker;

fn main() -> io::Result<()> {
    // create camera and camera information
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut first_mouse = true;
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
                shared::WINDOW_TITLE,
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
    let (shader_program, sprite_shader, skybox, models, tracker_colors) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // create shader program using shader.rs
        let shader_program = Shader::new("shaders/shader.vs", "shaders/shader.fs");

        let sprite_shader = Shader::new("shaders/sprite.vs", "shaders/sprite.fs");

        // textures for skybox
        let skybox = Skybox::new("resources/skybox/space", ".png");

        // actually allow transparency
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        // add all models to hashmap
        // -----------
        let mut models: HashMap<String, Model> = HashMap::new();
        models.insert("cube".to_string(), Model::new("resources/cube/cube.obj"));
        models.insert("sungod".to_string(), Model::new("resources/sungod/sungod.obj"));
        models.insert("asteroid".to_string(), Model::new("resources/new_asteroid/asteroid.obj"));

        let colors: [Vector4<f32>; 3] = [
            vec4(0.91797, 0.25, 0.2031, 1.0),
            vec4(0.2031, 0.7852, 0.91797, 1.0),
            vec4(0.3867, 0.9648, 0.0781, 1.0),
        ];

        (shader_program, sprite_shader, skybox, models, colors)
    };

    // create all objects
    let crosshair = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/crosshair.png");
        sprite.set_position(vec2(width as f32 / 2.0, height as f32 / 2.0));
        sprite.set_scale(Vector2::from_value(CROSSHAIR_SCALE));
        sprite
    };

    let empty_healthbar = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/emptyHealthBar.png");
        sprite.set_position(vec2(5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopLeft);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let full_healthbar = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/fullHealthBar.png");
        sprite.set_position(vec2(5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopLeft);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_0 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo0.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_1 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo1.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_2 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo2.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_3 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo3.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_4 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo4.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_5 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo5.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let ammo_6 = unsafe {
        let mut sprite = Sprite::new(screen_size, sprite_shader.id);
        sprite.set_texture("resources/ui_textures/ammo6.png");
        sprite.set_position(vec2(width as f32 - 5.0, height as f32 - 5.0));
        sprite.set_anchor(Anchor::TopRight);
        sprite.set_scale(Vector2::from_value(BAR_SCALE));
        sprite
    };

    let lasso = Lasso::new();
    
    let mut tracker = unsafe {
        let tracker = Tracker::new(sprite_shader.id, 1.0, vec2(width as f32, height as f32));
        tracker
    };

    // client ECS to be sent to server
    let mut client_ecs: Option<ClientECS> = None;

    // health component initialized
    let mut client_health = PlayerHealthComponent::default();
    let mut client_ammo = 0;

    // WINDOW LOOP
    // -----------
    loop {
        stream.set_nonblocking(true).unwrap();
        let mut input_component:PlayerInputComponent;
        let mut size_buf = [0 as u8; 4];
        let mut ready_sent = false; // prevents sending ready message twice
        let mut in_lobby = true;
        window.set_cursor_mode(glfw::CursorMode::Normal);
        println!("Press ENTER when ready to start game");
        // MENU LOOP
        while in_lobby {
            // poll enter key (ready button once GUI implemented)
            if !ready_sent && window.get_key(Key::Enter) == Action::Press {
                ready_sent = true;
                // send ready JSON (hardcoded for now)
                let ready_json = ReadyECS{ready:true};
                write_data(&mut stream, serde_json::to_string(&ready_json).unwrap());
                println!("Ready message sent!");
            }
            if window.get_key(Key::Escape) == Action::Press {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            }
            
            // TODO: render lobby frame
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            // poll server for ready message or ready-player updates
            let received = read_data(&mut stream);
            if received.len() > 0 {
                // ignore malformed input (probably leftover game state)
                let res : Result<LobbyECS, serde_json::Error> = serde_json::from_str(received.as_str());
                match res {
                    Ok(lobby_ecs) => {
                        if lobby_ecs.start_game {
                            println!("Game starting!");
                            let start_pos = &lobby_ecs.position_components[lobby_ecs.ids[client_id]];
                            camera.RotQuat = Quaternion::new(start_pos.qw, start_pos.qx, start_pos.qy, start_pos.qz);
                            camera.UpdateVecs();
                            client_ecs = None;
                            first_mouse = true;
                            in_lobby = false;
                        }
                    }
                    _ => ()
                }
            }
            
            // poll events
            window.swap_buffers();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    // Exit with code 0 upon window close
                    glfw::WindowEvent::Close => {
                        process::exit(0);
                    }
                    _ => {}
                }
            }
        }
      
        // GAME LOOP
        let mut in_game = true;
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        while in_game {
            input_component = PlayerInputComponent::default();

            let mut roll = false;

            // process inputs
            // --------------
            process_inputs(
                &mut window,
                &mut input_component,
                &mut roll
            );

            // events
            // ------
            process_events(
                &events,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
                &mut camera, roll
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

                        // setup player camera
                        let player_pos = vec3(
                            c_ecs.position_components[player_key].x,
                            c_ecs.position_components[player_key].y,
                            c_ecs.position_components[player_key].z
                        );
                        set_camera_pos(&mut camera, player_pos, &shader_program, width, height);

                        // draw models
                        for &renderable in &c_ecs.renderables {
                            if renderable != player_key {
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
                            
                                // setup scale matrix
                                let scale_mat = Matrix4::from_scale(c_ecs.model_components[renderable].scale);
                            
                                let model = pos_mat * scale_mat * rot_mat;
                                shader_program.set_mat4(c_str!("model"), &model);
                                let model_name = &c_ecs.model_components[renderable].modelname;
                                models[model_name].draw(&shader_program);
                            }
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
                                let rot_mat = Matrix4::from(Quaternion::new(player_qw, player_qx, player_qy, player_qz));

                                // setup scale matrix
                                let scale_mat = Matrix4::from_scale(c_ecs.model_components[player].scale);

                                let model = pos_mat * scale_mat * rot_mat;

                                let anchor_x = c_ecs.player_lasso_components[player].anchor_x;
                                let anchor_y = c_ecs.player_lasso_components[player].anchor_y;
                                let anchor_z = c_ecs.player_lasso_components[player].anchor_z;

                                // draw lasso
                                let lasso_p1 = model.transform_point(Point3::new(0.5, -1.0, 0.0));
                                let lasso_p2 = vec3(anchor_x, anchor_y, anchor_z);
                                lasso.draw_btw_points(lasso_p1.to_vec(), lasso_p2, &shader_program);
                            }

                            // draw trackers
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
                            in_game = false;
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

                crosshair.draw();

                if client_health.alive {
                    full_healthbar.draw();
                } else {
                    empty_healthbar.draw();
                }

                // draw ammo
                match client_ammo {
                    0 => ammo_0.draw(),
                    1 => ammo_1.draw(),
                    2 => ammo_2.draw(),
                    3 => ammo_3.draw(),
                    4 => ammo_4.draw(),
                    5 => ammo_5.draw(),
                    6 => ammo_6.draw(),
                    _ => ()
                }

                gl::DepthMask(gl::FALSE);
                tracker.draw_all_trackers(trackers);
                gl::DepthMask(gl::TRUE);
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            window.swap_buffers();
            glfw.poll_events();
        }
    }
}

fn set_camera_pos(camera: &mut Camera, pos: Vector3<f32>, shader_program: &Shader, width: u32, height: u32) {
    camera.Position.x = pos.x;
    camera.Position.y = pos.y;
    camera.Position.z = pos.z;

    unsafe {
        let view = camera.GetViewMatrix();
        shader_program.set_mat4(c_str!("view"), &view);
        let projection: Matrix4<f32> = perspective(
            Deg(camera.Zoom),
            width as f32 / height as f32,
            0.1,
            10000.0,
        );
        shader_program.set_mat4(c_str!("projection"), &projection);
    }
}

/// Event processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
    roll: bool,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_y = ypos;
                    *first_mouse = false;
                }

                let xoffset = xpos - *last_x;
                let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top

                *last_x = xpos;
                *last_y = ypos;

                camera.ProcessMouseMovement(xoffset, yoffset, roll);
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.ProcessMouseScroll(yoffset as f32);
            }
            // Exit with code 0 upon window close
            glfw::WindowEvent::Close => {
                process::exit(0);
            }
            _ => {}
        }
    }
}

// process input and edit client sending packet
fn process_inputs(
    window: &mut glfw::Window,
    input_component: &mut PlayerInputComponent,
    roll: &mut bool,
) {
    if window.get_key(Key::W) == Action::Press {
        input_component.w_pressed = true;
    }
    if window.get_key(Key::A) == Action::Press {
        input_component.a_pressed = true;
    }
    if window.get_key(Key::S) == Action::Press {
        input_component.s_pressed = true;
    }
    if window.get_key(Key::D) == Action::Press {
        input_component.d_pressed = true;
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        input_component.shift_pressed = true;
    }
    if window.get_key(Key::LeftControl) == Action::Press {
        input_component.ctrl_pressed = true;
    }
    if window.get_key(Key::R) == Action::Press {
        input_component.r_pressed = true;
    }
    if window.get_key(Key::Space) == Action::Press {
        *roll = true;
    }
    if window.get_mouse_button(MouseButton::Button1) == Action::Press {
        input_component.lmb_clicked = true;
    }
    if window.get_mouse_button(MouseButton::Button2) == Action::Press {
        input_component.rmb_clicked = true;
    }

    // TODO: add additional quit hotkey?

    // Defocuses window
    if window.get_key(Key::Escape) == Action::Press {
        window.set_cursor_mode(glfw::CursorMode::Normal);
    }

    // Refocus by clicking on window
    if window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
        window.set_cursor_mode(glfw::CursorMode::Disabled);
    }
}
