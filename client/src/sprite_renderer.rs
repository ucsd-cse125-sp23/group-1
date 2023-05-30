#![allow(dead_code)]

use crate::shader::Shader;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use std::f32::consts::PI;
use std::ffi::{c_void, CStr};
use std::{mem, ptr};

use cgmath::{vec2, vec3, Array, Matrix4, Rad, Vector2, Vector4, Zero};
use image::GenericImage;
use std::path::Path;

pub struct Texture {
    pub id: u32,
    pub size: Vector2<f32>,
}

pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

pub enum Anchor {
    BotLeft,
    BotRight,
    TopLeft,
    TopRight,
    Centered,
}

pub struct Sprite {
    pub projection: Matrix4<f32>,
    pub anchor: Anchor,
    pub transform: Transform,
    pub color: Vector4<f32>,
    pub quad_vao: u32,
    pub shader: Shader,
    pub has_texture: bool,
    pub texture: Texture,
}

impl Sprite {
    pub unsafe fn new(screen_size: Vector2<f32>, shader_id: u32) -> Sprite {
        let projection = cgmath::ortho(
            0.0,
            screen_size.x as f32,
            0.0,
            screen_size.y as f32,
            -1.0,
            1.0,
        );
        let mut sprite = Sprite {
            projection,
            anchor: Anchor::Centered,
            transform: Transform {
                position: Vector2::zero(),
                rotation: 0.0,
                scale: Vector2::from_value(1.0),
            },
            color: Vector4::from_value(1.0),
            quad_vao: 0,
            shader: Shader { id: shader_id },
            has_texture: false,
            texture: Texture {
                id: 0,
                size: Vector2::zero(),
            },
        };

        let vertices: [f32; 24] = [
            // pos    // tex
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 1.0,
            
            0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 0.0,
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

        gl::GenTextures(1, &mut self.texture.id);
        gl::BindTexture(gl::TEXTURE_2D, self.texture.id); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                                          // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        self.texture.size = vec2(img.width() as f32, img.height() as f32);
        self.has_texture = true;
    }

    pub fn set_color(&mut self, color: Vector4<f32>) {
        self.color = color;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.transform.rotation = rotation;
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.transform.position = position;
    }

    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.transform.scale = scale;
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    pub fn set_size(&mut self, size: Vector2<f32>) {
        self.transform.scale.x = size.x / self.texture.size.x;
        self.transform.scale.y = size.y / self.texture.size.y;
    }

    pub fn set_percentage_width(&mut self, screen_size: Vector2<f32>, percentage: f32) {
        let target_width = screen_size.x * percentage;
        let scale = target_width / self.texture.size.x;
        self.transform.scale = Vector2::from_value(scale);
    }

    pub fn set_percentage_height(&mut self, screen_size: Vector2<f32>, percentage: f32) {
        let target_width = screen_size.y * percentage;
        let scale = target_width / self.texture.size.y;
        self.transform.scale = Vector2::from_value(scale);
    }

    pub unsafe fn draw_from_corners(&self, top_left: Vector2<f32>, bottom_right: Vector2<f32>) {
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;
        self.draw_at_bot_left(top_left, vec2(width, height));
    }

    pub unsafe fn draw(&self) {
        let new_position;
        match self.anchor {
            Anchor::BotLeft => new_position = self.transform.position,
            Anchor::BotRight => {
                new_position = self.transform.position
                    + vec2(-self.texture.size.x * self.transform.scale.x, 0.0);
            }
            Anchor::TopLeft => {
                new_position = self.transform.position
                    + vec2(0.0, -self.texture.size.y * self.transform.scale.y);
            }
            Anchor::TopRight => {
                new_position = self.transform.position
                    + vec2(
                        -self.texture.size.x * self.transform.scale.x,
                        -self.texture.size.y * self.transform.scale.y,
                    );
            }
            Anchor::Centered => {
                new_position = self.transform.position
                    + vec2(
                        -self.texture.size.x * self.transform.scale.x / 2.0,
                        -self.texture.size.y * self.transform.scale.y / 2.0,
                    );
            }
        }
        self.draw_at_bot_left(
            new_position,
            vec2(
                self.texture.size.x * self.transform.scale.x,
                self.texture.size.y * self.transform.scale.y,
            ),
        );
    }

    pub unsafe fn draw_at_center(&self, position: Vector2<f32>, size: Vector2<f32>) {
        let new_position = position + vec2(-size.x / 2.0, -size.y / 2.0);
        self.draw_at_bot_left(new_position, size);
    }

    pub unsafe fn draw_at_top_left(&self, position: Vector2<f32>, size: Vector2<f32>) {
        let new_position = position + vec2(0.0, -size.y);
        self.draw_at_bot_left(new_position, size);
    }

    pub unsafe fn draw_at_top_right(&self, position: Vector2<f32>, size: Vector2<f32>) {
        let new_position = position + vec2(-size.x, -size.y);
        self.draw_at_bot_left(new_position, size);
    }

    pub unsafe fn draw_at_bot_right(&self, position: Vector2<f32>, size: Vector2<f32>) {
        let new_position = position + vec2(-size.x, 0.0);
        self.draw_at_bot_left(new_position, size);
    }

    pub unsafe fn draw_at_bot_left(&self, position: Vector2<f32>, size: Vector2<f32>) {
        self.shader.use_program();

        let mut model = Matrix4::from_translation(vec3(position.x, position.y, 0.0));

        model = model * Matrix4::from_translation(vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model = model * Matrix4::from_angle_z(Rad(self.transform.rotation * (PI / 180.0)));
        model = model * Matrix4::from_translation(vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        model = model * Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);

        self.shader.set_mat4(c_str!("projection"), &self.projection);
        self.shader.set_mat4(c_str!("model"), &model);
        self.shader.set_bool(c_str!("hasTexture"), self.has_texture);
        self.shader.set_vector4(c_str!("spriteColor"), &self.color);

        if self.has_texture {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
        }

        gl::BindVertexArray(self.quad_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }
}
