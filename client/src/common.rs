use cgmath::{Deg, Matrix4, perspective, Vector3};
use crate::camera::Camera;
use std::ffi::{CStr};
use std::process;
use std::sync::mpsc::Receiver;
use glfw::{Action, Key, MouseButton};
use shared::shared_components::PlayerInputComponent;
use crate::shader::Shader;

pub fn set_camera_pos(camera: &mut Camera, pos: Vector3<f32>, shader_program: &Shader, width: u32, height: u32) {
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
pub fn process_inputs(
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
    if window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
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
