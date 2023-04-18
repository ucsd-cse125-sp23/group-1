extern crate glfw;

// graphics
use self::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

// network
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use std::net::{TcpStream};
use std::str;

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

    // render loop
    // -----------
    while !window.should_close() {
        let mut client_data: ClientData = ClientData {
            client_id: 1,
            movement: String::from("no input"),
        };

        // events
        // -----
        process_events(&mut window, &events);

        process_inputs(&mut window, &mut client_data);

        // Send & receive client data
        if glfw.get_time() > last_send_time + SEND_TIME_INTERVAL {
            last_send_time = glfw.get_time();

            let j = serde_json::to_string(&client_data)?;
            stream.write(j.as_bytes())?;
            let mut data = [0 as u8; 50];
            match stream.read(&mut data) {
                Ok(size) => {
                    let message: &str = str::from_utf8(&data[0..size]).unwrap();
                    if message.len() > 0 {
                        let value: ClientData = serde_json::from_str(message).unwrap();
                        println!("received: {}", value.movement);
                    }
                }
                _ => {}
            }
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