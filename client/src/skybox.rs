use crate::shader::Shader;
use image::GenericImage;
use std::ffi::{c_void, CStr};
use std::{mem, ptr};
use std::path::Path;
use cgmath::{Matrix4};
use gl::types::*;

pub struct Skybox {
    pub shader_program: Shader,
    vao: GLuint,
    texture_id: u32
}

impl Skybox {
    pub unsafe fn new(path: &str, format: &str) -> Skybox {
        let mut skybox = Skybox {
            shader_program: Shader { id: 0 },
            vao: 0,
            texture_id: 0
        };
        skybox.shader_program = Shader::new("shaders/skybox.vs",
                                            "shaders/skybox.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let skybox_vertices: [f32; 108] = [
            // positions
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0
        ];

        // skybox VAO
        let (mut skybox_vao, mut skybox_vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut skybox_vao);
        gl::GenBuffers(1, &mut skybox_vbo);
        gl::BindVertexArray(skybox_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, skybox_vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (skybox_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &skybox_vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

        skybox.vao = skybox_vao;
        skybox.texture_id = load_cubemap(path, format);

        // shader configuration
        // --------------------
        skybox.shader_program.use_program();
        skybox.shader_program.set_int(c_str!("skybox"), 0);

        skybox
    }

    pub unsafe fn draw(&self, mut view: Matrix4<f32>, projection: Matrix4<f32>) {
        // draw skybox as last
        gl::DepthFunc(gl::LEQUAL);  // change depth function so depth test passes when values are equal to depth buffer's content
        self.shader_program.use_program();
        // remove translation from the view matrix
        view.w[0] = 0.0;
        view.w[1] = 0.0;
        view.w[2] = 0.0;
        self.shader_program.set_mat4(c_str!("view"), &view);
        self.shader_program.set_mat4(c_str!("projection"), &projection);
        // skybox cube
        gl::BindVertexArray(self.vao);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        gl::BindVertexArray(0);
        gl::DepthFunc(gl::LESS); // set depth function back to default
    }
}

/// loads a cubemap texture from 6 individual texture faces
/// order:
/// +X (right)
/// -X (left)
/// +Y (top)
/// -Y (bottom)
/// +Z (front)
/// -Z (back)
/// -------------------------------------------------------
unsafe fn load_cubemap(path: &str, format: &str) -> u32 {
    let faces = [
        format!("{}/{}{}", path, "right", format),
        format!("{}/{}{}", path, "left", format),
        format!("{}/{}{}", path, "top", format),
        format!("{}/{}{}", path, "bottom", format),
        format!("{}/{}{}", path, "front", format),
        format!("{}/{}{}", path, "back", format),
    ];

    let mut color_format = gl::RGB;
    if format == ".png" {
        color_format = gl::RGBA;
    } else if format == ".jpg" {
        color_format = gl::RGB;
    }

    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

    for (i, face) in faces.iter().enumerate() {
        let img = image::open(&Path::new(face)).expect("Cubemap texture failed to load");

        let data = img.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            color_format,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
    }

    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_R,
        gl::CLAMP_TO_EDGE as i32,
    );

    texture_id
}
