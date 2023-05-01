#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use russimp::{mesh, material, texture};
use russimp::texture::TextureType;
use tobj;

use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader::Shader;

#[derive(Default)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,
    // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        model
    }

    pub fn new_from_assimp(meshes: &Vec<mesh::Mesh>, materials: &Vec<material::Material>) -> Model {
        let mut model = Model::default();
        model.load_model_from_assimp(meshes, materials);
        model
    }

    fn load_model_from_assimp(&mut self, meshes: &Vec<mesh::Mesh>, materials: &Vec<material::Material>) {
        for mesh in meshes {
            let num_vertices = mesh.vertices.len();

            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let mut indices = Vec::new();

            for face in &mesh.faces {
                indices.extend(face.0.clone());
            }

            // create vertices
            let (p, n, t_option) = (&mesh.vertices, &mesh.normals, &mesh.texture_coords);
            let t = match &t_option[0] {
                Some(tex) => tex,
                None => panic!("No texture coordinates!")
            };
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: vec3(p[i].x, p[i].y, p[i].z),
                    normal: vec3(n[i].x, n[i].y, n[i].z),
                    tex_coords: vec2(t[i].x, t[i].y),
                    ..Vertex::default()
                })
            }

            // process material
            let mut textures: Vec<Texture> = Vec::new();
            let material_id = mesh.material_index as usize;
            let material = &materials[material_id];

            // 1. diffuse map
            if !material.textures[&TextureType::Diffuse].is_empty() {
                let texture =
                    self.load_material_texture_from_assimp(&material.textures[&TextureType::Diffuse], "texture_diffuse");
                textures.push(texture);
            }
            // // 2. specular map
            // if !material.specular_texture.is_empty() {
            //     let texture =
            //         self.load_material_texture(&material.specular_texture, "texture_specular");
            //     textures.push(texture);
            // }
            // // 3. normal map
            // if !material.normal_texture.is_empty() {
            //     let texture =
            //         self.load_material_texture(&material.normal_texture, "texture_normal");
            //     textures.push(texture);
            // }
            // // NOTE: no height maps

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material_texture_from_assimp(&mut self, aiTexture: &Vec<texture::Texture>, type_name: &str) -> Texture {
        println!("{}", &aiTexture[0].path);
        let tmp = match &aiTexture[0].data {
            Some(data) => data,
            _ => panic!("no data")
        };

        {
            let texture = self.textures_loaded.iter().find(|t| t.path == aiTexture[0].path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { texture_from_file(&aiTexture[0].path, &self.directory) },
            type_: type_name.into(),
            path: aiTexture[0].path.clone(),
        };
        self.textures_loaded.push(texture.clone());
        texture
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe {
                mesh.draw(shader);
            }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, path: &str) {
        let path = Path::new(path);

        // retrieve the directory path of the filepath
        self.directory = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    tex_coords: vec2(t[i * 2], t[i * 2 + 1]),
                    ..Vertex::default()
                })
            }

            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material_texture(&mut self, path: &str, type_name: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { texture_from_file(path, &self.directory) },
            type_: type_name.into(),
            path: path.into(),
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}

unsafe fn texture_from_file(path: &str, directory: &str) -> u32 {
    let filename = format!("{}/{}", directory, path);

    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);

    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };

    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, texture_id);
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

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
}
