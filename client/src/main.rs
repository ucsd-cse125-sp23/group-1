mod shader;
mod macros;
mod camera;

// graphics
extern crate glfw;
extern crate gl;

use self::glfw::{Context, Key, Action};
use self::gl::types::*;
use cgmath::{Matrix4, Deg, vec3, perspective, Point3, Vector3, InnerSpace};

use std::sync::mpsc::Receiver;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;
use std::mem;

use crate::shader::Shader;
use crate::camera::*;

// network
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::str;
use shared::shared_components::*;

// graphics settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() -> std::io::Result<()> {
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
    let (shader_program, vao, cube_pos) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // create shader program using shader.rs
        let shader_program = Shader::new(
            "shaders/shader.vs",
            "shaders/shader.fs",
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 24] = [
            0.5, 0.5, -0.5,     // top right
            0.5, -0.5, -0.5,    // bottom right
            -0.5, -0.5, -0.5,   // bottom left
            -0.5, 0.5, -0.5,    // top left

            0.5, 0.5, 0.5,      // top right
            0.5, -0.5, 0.5,     // bottom right
            -0.5, -0.5, 0.5,    // bottom left
            -0.5, 0.5, 0.5      // top left
        ];
        let indices = [
            // bottom
            0, 1, 3,
            1, 2, 3,
            // top
            5, 4, 7,
            7, 6, 5,
            // right 
            5, 1, 0,
            0, 4, 5,
            // left
            7, 3, 2,
            2, 6, 7,
            // front
            5, 6, 2,
            2, 1, 5,
            // back
            0, 3, 7,
            7, 4, 0
        ];

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
            vec3(-1.3, 1.0, -1.5)];

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // remember: do NOT unbind the EBO while a VAO is active as the bound element buffer object IS stored in the VAO; keep the EBO bound.
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao, cube_pos)
    };

    // client ECS to be sent to server
    let mut client_ecs = ClientECS::default();

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
        let send_size = j.len() as u32;
        let send = [u32::to_be_bytes(send_size).to_vec(), j.clone().into_bytes()].concat();
        match stream.write(&send) {
            Ok(_) => (),
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
                    client_ecs = value;
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
            let x = client_ecs.position_components[].x;
            let y = client_ecs.position_components[].y;
            let z = client_ecs.position_components[].z;
            let model_pos = vec3(x,y,z);

            // create transformations and pass them to vertex shader
            let mut model = Matrix4::from_angle_x(Deg(-45.));
            model = Matrix4::from_translation(model_pos) * model;
            shader_program.set_mat4(c_str!("model"), &model);

            let view = camera.GetViewMatrix();
            shader_program.set_mat4(c_str!("view"), &view);

            // let view = Matrix4::look_at(cam_pos, cam_look, cam_up);
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32 , 0.1, 100.0);
            shader_program.set_mat4(c_str!("projection"), &projection);

            // camera coordinates calculation: u, v, w: points away from camera

            // let cam_point = cam_look - cam_pos;

            gl::BindVertexArray(vao); // seeing as we only have a single vao there's no need to bind it every time, but we'll do so to keep things a bit more organized
            for (i, position) in cube_pos.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model;
                if i == 0 {
                    model = Matrix4::from_translation(model_pos);
                } else {
                    model = Matrix4::from_translation(*position);
                }
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                shader_program.set_mat4(c_str!("model"), &model);

                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
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