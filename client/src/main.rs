mod shader;
mod macros;
mod camera;
mod mesh;
mod model;
mod skybox;

use std::collections::HashMap;

// graphics
extern crate glfw;
extern crate gl;

use self::glfw::{Context, Key, MouseButton, Action};
use cgmath::{Matrix4, Quaternion, Deg, vec3, perspective, Point3, Vector3};

use std::sync::mpsc::Receiver;
use std::ffi::{CStr, c_void};
use core::{mem::{size_of, size_of_val}};

use crate::shader::Shader;
use crate::camera::*;
use crate::model::Model;
use crate::skybox::Skybox;

// network
use std::io::{Read, self};
use std::net::{TcpStream};
use std::str;
use std::process;
use shared::shared_components::*;
use shared::shared_functions::*;

fn main() -> std::io::Result<()> {
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
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
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
    let mut stream = TcpStream::connect(shared::SERVER_ADDR.to_string() + ":" + &shared::PORT.to_string())?;

    // receive and save client id
    let mut read_buf = [0u8, 1];
    stream.read(&mut read_buf).unwrap();
    let client_id = read_buf[0] as usize;
    println!("client id: {}", client_id);

    stream.set_nonblocking(true).expect("Failed to set stream as nonblocking");

    // Set up OpenGL shaders
    let (shader_program, hud_shader, skybox, models) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // create shader program using shader.rs
        let shader_program = Shader::new(
            "shaders/shader.vs",
            "shaders/shader.fs",
        );

        // create HUD shader
        let hud_shader = Shader::new(
            "shaders/hud.vs",
            "shaders/hud.fs",
        );

        // textures for skybox
        let skybox = Skybox::new("resources/skybox/space", ".png");

        // actually allow transparency
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        // add all models to hashmap
        // -----------
        let mut models: HashMap<String,Model> = HashMap::new();
        models.insert("cube".to_string(), Model::new("resources/cube/cube.obj"));
        models.insert("sungod".to_string(), Model::new("resources/sungod/sungod.obj"));
        models.insert("asteroid".to_string(), Model::new("resources/new_asteroid/asteroid.obj"));

        (shader_program, hud_shader, skybox, models)
    };

    // client ECS to be sent to server
    let mut client_ecs: Option<ClientECS> = None;

    // health component initialized
    let mut client_health = PlayerHealthComponent::default();

    // set up HUD renderer
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao as *mut u32);
        gl::GenBuffers(1, &mut vbo);
        
        // define crosshair vertices (TEMPORARY)
        // coords are relative to screen size -- currently 640x480
        // TODO: re-implement with textured quad
        let vertices: [f32; 8] = [
        -0.0375, -0.0,
         0.0375, -0.0,
         0.0,   0.05,
         0.0,  -0.05,
        ];

        // 1. bind Vertex Array Object
        gl::BindVertexArray(vao);
        // 2. copy array into a buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );
        // 3. set vertex attribute pointers
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (2 * size_of::<f32>()) as i32, 0 as *mut c_void);
        gl::EnableVertexAttribArray(0);
    }
 
    
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
                            let start_pos = &lobby_ecs.position_components[lobby_ecs.players[client_id]];
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
            process_inputs(&mut window, &mut input_component, &mut roll);

            // events
            // ------
            process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera, roll);

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
                let size:u32;
                match stream.peek(&mut size_buf) {
                    Ok(4) => {
                        // big-endian for networks. it's tradition, dammit!
                        size = u32::from_be_bytes(size_buf);
                    },
                    Ok(_) => {
                        // incomplete size field, wait for next tick
                        break;
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to read message size from server: {}",e);
                        // TODO: handle lost client
                        break;
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
                    },
                    Ok(_) => {
                        break;
                    },
                    Err(e) => {
                        eprintln!("Failed to read message from server: {}",e);
                    },
                }
            }

            // render
            // ------
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // activate shader
                shader_program.use_program();

                // NEEDS TO BE REWORKED FOR MENU STATE
                match &client_ecs {
                    Some(c_ecs) => {
                        let player_key = c_ecs.players[client_id];

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

                        let player_pos = vec3(c_ecs.position_components[player_key].x,
                            c_ecs.position_components[player_key].y,
                            c_ecs.position_components[player_key].z);
                        set_camera_pos(&mut camera, player_pos, &shader_program, width, height);

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
                            
                                // setup scale matrix (skip for now)
                                let scale_mat = Matrix4::from_scale(c_ecs.model_components[renderable].scale);
                            
                                let model = pos_mat * scale_mat * rot_mat;
                                shader_program.set_mat4(c_str!("model"), &model);
                                let model_name = &c_ecs.model_components[renderable].modelname;
                                models[model_name].draw(&shader_program);
                            }
                        }

                        // game has ended
                        if c_ecs.game_ended {
                            for (i, player) in c_ecs.players.iter().enumerate() {
                                if c_ecs.health_components[*player].alive {
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
                let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), width as f32 / height as f32 , 0.1, 100.0);
                skybox.draw(camera.GetViewMatrix(), projection);

                // DRAW HUD
                hud_shader.use_program();
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::LINES, 0, 4);
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
        
        let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), width as f32 / height as f32 , 0.1, 10000.0);
        shader_program.set_mat4(c_str!("projection"), &projection);
    }
}

/// Event processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>,
                      first_mouse: &mut bool,
                      last_x: &mut f32,
                      last_y: &mut f32,
                      camera: &mut Camera,
                      roll: bool) {
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
fn process_inputs(window: &mut glfw::Window, input_component: &mut PlayerInputComponent, roll: &mut bool) {
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