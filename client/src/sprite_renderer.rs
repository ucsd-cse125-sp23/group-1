use crate::shader::Shader;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use std::ffi::{CStr, c_void};
use std::{mem, ptr};
use std::f32::consts::PI;

use std::path::Path;
use cgmath::{Matrix4, Rad, vec3, Vector2, Vector3, Vector4};
use image::GenericImage;

pub struct Sprite {
    pub quad_vao: u32,
    pub shader: Shader,
    pub has_texture: bool,
    pub texture_id: u32,
}

impl Sprite {
    pub unsafe fn new() -> Sprite {
        let mut sprite = Sprite {
            quad_vao: 0,
            shader: Shader { id: 0 },
            has_texture: false,
            texture_id: 0,
        };

        let vertices: [f32; 24] = [
            // pos    // tex
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
        ];

        let mut vbo = 0;

        gl::GenVertexArrays(1, &mut sprite.quad_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
        let data = &vertices[0] as *const f32 as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindVertexArray(sprite.quad_vao);
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        sprite
    }

    pub unsafe fn set_texture(&mut self, path: &str) {
        if self.has_texture {
            panic!("Sprite already has texture");
        }

        let mut format = gl::RGB;
        if path.ends_with(".png") {
            format = gl::RGBA;
        }

        gl::GenTextures(1, &mut self.texture_id);
        gl::BindTexture(gl::TEXTURE_2D, self.texture_id); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       format as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       format,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        self.has_texture = true;
    }

    pub unsafe fn draw(&self, projection: &Matrix4<f32>, position: Vector2<f32>, size: Vector2<f32>, rotate: f32, color: Vector4<f32>) {
        self.shader.use_program();

        let mut model = Matrix4::from_translation(vec3(position.x, position.y, 0.0));

        model = model * Matrix4::from_translation(vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model = model * Matrix4::from_angle_z(Rad(rotate * (PI / 180.0)));
        model = model * Matrix4::from_translation(vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        model = model * Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("model"), &model);
        self.shader.set_bool(c_str!("hasTexture"), self.has_texture);
        self.shader.set_vector4(c_str!("spriteColor"), &color);

        if self.has_texture {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }

        gl::BindVertexArray(self.quad_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }
}
