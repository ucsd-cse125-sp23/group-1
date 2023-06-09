use cgmath::{Deg, Matrix4, perspective, Vector3};
use shared::shared_functions::write_data;
use crate::camera::Camera;
use std::ffi::{CStr};
use std::net::TcpStream;
use std::process;
use std::sync::mpsc::Receiver;
use glfw::{Action, Key, Window, Glfw};
use shared::shared_components::{PlayerInputComponent, ReadyECS};
use crate::shader::Shader;

pub fn set_camera_pos(camera: &mut Camera, pos: Vector3<f32>, shader_program: &Shader, width: u32, height: u32) {
    camera.Position.x = pos.x;
    camera.Position.y = pos.y;
    camera.Position.z = pos.z;

    unsafe {
        let view = camera.GetViewMatrix();
        shader_program.set_mat4(c_str!("view"), &view);
        let projection: Matrix4<f32> = perspective(
            Deg(camera.Fov),
            width as f32 / height as f32,
            0.1,
            10000.0,
        );
        shader_program.set_mat4(c_str!("projection"), &projection);
    }
}

pub fn process_events_lobby(
    events: &Receiver<(f64, glfw::WindowEvent)>
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            // Exit with code 0 upon window close
            glfw::WindowEvent::Close => {
                process::exit(0);
            }
            _ => {}
        }
    }
}

/// Event processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_events_game(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
    roll: bool,
    is_focused: bool
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                if !is_focused {
                    return;
                }
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
            // Exit with code 0 upon window close
            glfw::WindowEvent::Close => {
                process::exit(0);
            }
            _ => {}
        }
    }
}

pub fn process_events_game_over(
    events: &Receiver<(f64, glfw::WindowEvent)>
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            // Exit with code 0 upon window close
            glfw::WindowEvent::Close => {
                process::exit(0);
            }
            _ => {}
        }
    }
}

pub fn process_inputs_lobby(
    window: &mut glfw::Window,
    ready_sent: &mut bool,
    first_enter: &mut bool,
    stream: &mut TcpStream
) {
    if !*first_enter && !*ready_sent && window.get_key(Key::Enter) == Action::Press {
        *ready_sent = true;
        // send ready JSON (hardcoded for now)
        let ready_json = ReadyECS{ready:true};
        write_data(stream, serde_json::to_string(&ready_json).unwrap());
    }
    if window.get_key(Key::Enter) == Action::Release {
        *first_enter = false;
    }
}

// process input and edit client sending packet
pub fn process_inputs_game(
    window: &mut glfw::Window,
    input_component: &mut PlayerInputComponent,
    roll: &mut bool,
    zoomed: &mut bool,
    mmb_clicked: &mut bool,
    first_click: &mut bool,
    is_focused: bool
) {
    if !is_focused {
        return;
    }

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
    if window.get_mouse_button(glfw::MouseButtonRight) == Action::Press {
        input_component.rmb_clicked = true;
    }
    if !*first_click && window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
        input_component.lmb_clicked = true;
    }
    if window.get_mouse_button(glfw::MouseButtonLeft) == Action::Release {
        *first_click = false;
    }
    if !*mmb_clicked && window.get_mouse_button(glfw::MouseButtonMiddle) == Action::Press {
        *zoomed = !*zoomed;
        *mmb_clicked = true;
    }
    if *mmb_clicked && window.get_mouse_button(glfw::MouseButtonMiddle) == Action::Release {
        *mmb_clicked = false;
    }

    // TODO: add additional quit hotkey?
}

pub fn process_inputs_game_over(
    window: &mut glfw::Window,
    first_enter: &mut bool
) -> bool {
    if !*first_enter && window.get_key(Key::Enter) == Action::Press {
        *first_enter = true;
        return true;
    }
    return false;
}

pub fn set_fullscreen(fullscreen: bool, glfw: &mut Glfw, window: &mut Window, width: &mut u32, height: &mut u32, saved_xpos: &mut i32, saved_ypos: &mut i32, saved_width: &mut u32, saved_height: &mut u32, refresh_rate: u32) {
    if fullscreen {
        *saved_height = *height;
        *saved_width = *width;
        *saved_xpos = window.get_pos().0;
        *saved_ypos = window.get_pos().1;
        glfw.with_primary_monitor(|_, m| {
            let mode = glfw::Monitor::get_video_mode(m.expect("access monitor for video mode")).expect("failed to get video mode");
            *width = mode.width as u32;
            *height = mode.height as u32;
        });
        window.set_decorated(false);
        window.set_monitor(glfw::WindowMode::Windowed, 0, 0, *width, *height, Some(refresh_rate));
    } else {
        window.set_decorated(true);
        window.set_monitor(glfw::WindowMode::Windowed, *saved_xpos, *saved_ypos, *saved_width * 2, *saved_height * 2, Some(refresh_rate));
        *height = *saved_height;
        *width = *saved_width;
    }
}
