mod shader;
mod macros;

// graphics
extern crate glfw;
extern crate gl;

use self::glfw::{Context, Key, Action};
use self::gl::types::*;

use std::convert::identity;
use std::sync::mpsc::Receiver;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;
use std::mem;

use cgmath::{Matrix4, Deg, vec3, perspective, Matrix, Vector3, SquareMatrix, Point3};

// network
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use std::net::{TcpStream};
use std::str;
use crate::shader::Shader;

// graphics settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// network settings
const SEND_TIME_INTERVAL: f64 = 0.3;

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

#[derive(Serialize, Deserialize)]
struct Coords {
    x: f32,         // vec3() is f32, not f64
    y: f32,
    z: f32,
}

fn main() -> std::io::Result<()> {
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
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create network TcpStream
    let mut stream = TcpStream::connect("localhost:8080")?;
    let mut last_send_time: f64 = 0.0;

    // Set up OpenGL shaders
    let (shader_program, vao) = unsafe {
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

        (shader_program, vao)
    };

    let mut coords = Coords {x:0.0, y:0.0, z:0.0};
    let mut cam_pos = vec3(0., 0., 3.);
    let mut cam_look = vec3(0., 0., -1.);
    let mut cam_up = vec3(0., 1., 0.);

    // set the projection matrix
    // let projection = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    // unsafe { shader_program.set_mat4(c_str!("projection"), &projection); }

    // render loop
    // -----------
    while !window.should_close() {
        let mut client_data: ClientData = ClientData {
            client_id: 1,
            movement: String::from("no input"),
        };

        // events
        // ------
        process_events(&mut window, &events);

        // process inputs
        // --------------
        process_inputs(&mut window, &mut client_data);

        // Send & receive client data
        if glfw.get_time() > last_send_time + SEND_TIME_INTERVAL {
            last_send_time = glfw.get_time();

            let j = serde_json::to_string(&client_data)?;
            stream.write(j.as_bytes())?;

            let mut buf = [0 as u8; 128];
            let size = stream.read(&mut buf)?;
            let message: &str = str::from_utf8(&buf[0..size]).unwrap();
            coords = serde_json::from_str(message).unwrap();
            println!("{}", message);
        }

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // activate shader
            shader_program.use_program();

            // update model_pos based on message from server
            let model_pos = vec3(coords.x, coords.y, coords.z);

            // create transformations
            let mut model = Matrix4::from_angle_x(Deg(-55.));//Matrix4::identity();////
            model = model * Matrix4::from_translation(model_pos);
            let view = Matrix4::from_translation(vec3(0., 0., -3.));
            // let view = Matrix4::look_at(cam_pos, cam_look, cam_up);
            let projection = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            // camera coordinates calculation: u, v, w: points away from camera

            // let cam_point = cam_look - cam_pos;

            // retrieve the matrix uniform locations (address of the matrices)
            let model_loc = gl::GetUniformLocation(shader_program.id, c_str!("model").as_ptr());
            let view_loc = gl::GetUniformLocation(shader_program.id, c_str!("view").as_ptr());

            // pass matrices to vertex shader (3 different ways)
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            shader_program.set_mat4(c_str!("projection"), &projection);

            gl::BindVertexArray(vao); // seeing as we only have a single vao there's no need to bind it every time, but we'll do so to keep things a bit more organized
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

// process input and edit client sending packet
fn process_inputs(window: &mut glfw::Window, client_data: &mut ClientData) {
    if window.get_key(Key::Down) == Action::Press {
        client_data.movement = String::from("down");
    } else if window.get_key(Key::Up) == Action::Press {
        client_data.movement = String::from("up");
    } else if window.get_key(Key::Left) == Action::Press {
        client_data.movement = String::from("left");
    } else if window.get_key(Key::Right) == Action::Press {
        client_data.movement = String::from("right");
    }
}