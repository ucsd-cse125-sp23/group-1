mod shader;
mod macros;
mod camera;
mod mesh;
mod model;

// graphics
use glfw::{Context, Key, Action};
use cgmath::{Matrix4, Deg, vec3, perspective, Point3, Vector3, InnerSpace};
use gltf::Gltf;

use std::sync::mpsc::Receiver;
use std::ffi::CStr;

use crate::shader::Shader;
use crate::camera::*;
use crate::model::Model;

// network
use std::io::{Read, Write, self};
use std::net::{TcpStream};
use std::str;
use shared::shared_components::*;

// graphics settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() -> std::io::Result<()> {
    let gltf_file = Gltf::open("resources/test_skeleton.gltf");

    let gltf = match gltf_file {
        Ok(gltf) => gltf,
        Err(err) => panic!("Problem reading gltf: {:?}", err),
    };

    for scene in gltf.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children",
                node.index(),
                node.children().count(),
            );
        }
    }

    return Result::Ok(());

    // create camera and camera information
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create network TcpStream
    let mut stream = TcpStream::connect("localhost:8080")?;
    stream.set_nonblocking(true).expect("Failed to set stream as nonblocking");

    // Set up OpenGL shaders
    let (shader_program, models, cube_pos) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // create shader program using shader.rs
        let shader_program = Shader::new(
            "shaders/shader.vs",
            "shaders/shader.fs",
        );

        // world space positions of our cubes
        let cube_pos: [Vector3<f32>; 10] = [vec3(0.0, 0.0, 0.0),
            vec3(2.0, 5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3(2.4, -0.4, -3.5),
            vec3(-1.7, 3.0, -7.5),
            vec3(1.3, -2.0, -2.5),
            vec3(1.5, 2.0, -2.5),
            vec3(1.5, 0.2, -1.5),
            vec3(-1.3, 1.0, -1.5)
        ];

        // load models
        // -----------
        let models = Model::new("resources/cube/cube.obj");

        (shader_program, models, cube_pos)
    };

    // client ECS to be sent to server
    // let mut client_ecs = ClientECS::default();
    let mut client_ecs: Option<ClientECS> = None;

    // render loop
    // -----------
    while !window.should_close() {
        // create player input component
        let mut input_component = PlayerInputComponent::default();

        // events
        // ------
        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        // process inputs
        // --------------
        process_inputs(&mut window, &mut input_component);

        // set camera front of input_component
        input_component.camera_front_x = camera.Front.x;
        input_component.camera_front_y = camera.Front.y;
        input_component.camera_front_z = camera.Front.z;

        // Send & receive client data
        let j = serde_json::to_string(&input_component).expect("Input component serialization error");
        let send_size = j.len() as u32 + 4;
        let send = [u32::to_be_bytes(send_size).to_vec(), j.clone().into_bytes()].concat();
        match stream.write(&send) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => eprintln!("Error sending input: {:?}", e),
        };

        // let mut buf = [0 as u8; 128];
        // let size = stream.read(&mut buf)?;
        // if size > 0 {
        //     let message: &str = str::from_utf8(&buf[0..size]).unwrap();
        //     coords = serde_json::from_str(message).unwrap();
        //     println!("{}", message);
        // }

        loop {
            let mut size_buf = [0 as u8; 4];
            let size:u32;
            match stream.peek(&mut size_buf) {
                Ok(4) => {
                    // it's tradition, dammit!
                    size = u32::from_be_bytes(size_buf);
                },
                Ok(_) => {
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

            // update model_pos based on message from server
            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;
            match &client_ecs {
                Some(c_ecs) => {
                    x = c_ecs.position_components[c_ecs.temp_entity].x;
                    y = c_ecs.position_components[c_ecs.temp_entity].y;
                    z = c_ecs.position_components[c_ecs.temp_entity].z;
                }
                None => ()
            }
            let model_pos = vec3(x,y,z);

            // create transformations and pass them to vertex shader
            let mut model_mat = Matrix4::from_angle_x(Deg(-45.));
            model_mat = Matrix4::from_translation(model_pos) * model_mat;
            shader_program.set_mat4(c_str!("model"), &model_mat);

            let view = camera.GetViewMatrix();
            shader_program.set_mat4(c_str!("view"), &view);

            // let view = Matrix4::look_at(cam_pos, cam_look, cam_up);
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32 , 0.1, 100.0);
            shader_program.set_mat4(c_str!("projection"), &projection);

            // camera coordinates calculation: u, v, w: points away from camera

            // let cam_point = cam_look - cam_pos;

            for (i, position) in cube_pos.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model;
                if i == 0 {
                    model = Matrix4::from_translation(model_pos);
                } else {
                    model = Matrix4::from_translation(*position);
                }
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_scale(0.5);
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                shader_program.set_mat4(c_str!("model"), &model);

                models.draw(&shader_program);
            }
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

/// Event processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>,
                      first_mouse: &mut bool,
                      last_x: &mut f32,
                      last_y: &mut f32,
                      camera: &mut Camera) {
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

                camera.ProcessMouseMovement(xoffset, yoffset, true);
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.ProcessMouseScroll(yoffset as f32);
            }
            _ => {}
        }
    }
}

// process input and edit client sending packet
fn process_inputs(window: &mut glfw::Window, input_component: &mut PlayerInputComponent) {
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

    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}