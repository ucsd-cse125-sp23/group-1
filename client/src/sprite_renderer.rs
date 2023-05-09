use crate::shader::Shader;
use gl::types::GLsizei;
use std::ffi::{CStr, c_void};
use std::mem;
use std::mem::{size_of, size_of_val};
use cgmath::{Matrix4, Rad, SquareMatrix, vec3, Vector2, Vector3};

pub struct Sprite {
    pub quad_vao: u32,
    pub shader: Shader,
}

impl Sprite {
    pub unsafe fn new() -> Sprite {
        let mut sprite = Sprite {
            quad_vao: 0,
            shader: Shader { id: 0 },
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
        let size = size_of_val(&vertices) as isize;
        let data = &vertices[0] as *const f32 as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindVertexArray(sprite.quad_vao);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * size_of::<f32>()) as GLsizei,
            0 as *const c_void,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        sprite
    }

    pub unsafe fn draw(&self, position: Vector2<f32>, size: Vector2<f32>, rotate: f32, color: Vector3<f32>) {
        self.shader.use_program();

        let mut model = Matrix4::identity();
        model = Matrix4::from_translation(vec3(position.x, position.y, 0.0));

        model = Matrix4::from_translation(vec3(0.5 * size.x, 0.5 * size.y, 0.0)) * model;
        model = Matrix4::from_angle_z(Rad(rotate)) * model;
        model = Matrix4::from_translation(vec3(-0.5 * size.x, -0.5 * size.y, 0.0)) * model;

        model = Matrix4::from_nonuniform_scale(size.x, size.y, 1.0) * model;

        self.shader.set_mat4(c_str!("model"), &model);
        self.shader.set_vector3(c_str!("spriteColor"), &color);

        gl::ActiveTexture(gl::TEXTURE0);

        gl::BindVertexArray(self.quad_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }
}
