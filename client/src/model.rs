#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use cgmath::InnerSpace;
use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use crate::mesh::{ Mesh, Texture, Vertex };
use crate::shader::Shader;

#[derive(Default)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,   // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        model
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.draw(shader); }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, path: &str) {
        let path = Path::new(path);

        // retrieve the directory path of the filepath
        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            
            let mut tans: Vec<f32> = vec![0.; num_vertices * 3];
            let mut bitans: Vec<f32> = vec![0.; num_vertices * 3];

            if n.len() > 0 && n.len() < num_vertices * 3 {
                eprintln!("missing normals for {} model, expected {} got {}. using default (0, 0, 0)", model.name, num_vertices*3, n.len());
            }

            if t.len() > 0 && t.len() < num_vertices * 2 {
                eprintln!("missing textures for {} model, expected {} got {}. using default (0, 0)", model.name, num_vertices*2, t.len());
            }

            // calculate tangents and bitangents
            let num_tri = mesh.indices.len() / 3;
            for i in 0..num_tri {
                // get triangle vertex and texture coordinates
                let i1 = mesh.indices[i*3+0] as usize;
                let i2 = mesh.indices[i*3+1] as usize;
                let i3 = mesh.indices[i*3+2] as usize;
                let p1 = vec3(p[i1*3+0], p[i1*3+1], p[i1*3+2]);
                let p2 = vec3(p[i2*3+0], p[i2*3+1], p[i2*3+2]);
                let p3 = vec3(p[i3*3+0], p[i3*3+1], p[i3*3+2]);
                let uv1 = if i2*2+1 >= t.len() {vec2(0.,0.)} else {vec2(t[i1*2+0], t[i1*2+1])};
                let uv2 = if i2*2+1 >= t.len() {vec2(0.,0.)} else {vec2(t[i2*2+0], t[i2*2+1])};
                let uv3 = if i3*2+1 >= t.len() {vec2(0.,0.)} else {vec2(t[i3*2+0], t[i3*2+1])};

                // calculate tangent/bitangent vectors of both triangles
                let edge1 = p2 - p1;
                let edge2 = p3 - p1;
                let delta_uv1 = uv2 - uv1;
                let delta_uv2 = uv3 - uv1;
                let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
                let tanx = f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x);
                let tany = f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y);
                let tanz = f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z);
                let bitanx = f * (-delta_uv2.x * edge1.x + delta_uv1.x * edge2.x);
                let bitany = f * (-delta_uv2.x * edge1.y + delta_uv1.x * edge2.y);
                let bitanz = f * (-delta_uv2.x * edge1.z + delta_uv1.x * edge2.z);
                tans[i1*3+0] = tanx; tans[i1*3+1] = tany; tans[i1*3+2] = tanz;
                tans[i2*3+0] = tanx; tans[i2*3+1] = tany; tans[i2*3+2] = tanz;
                tans[i3*3+0] = tanx; tans[i3*3+1] = tany; tans[i3*3+2] = tanz;
                bitans[i1*3+0] = bitanx; bitans[i1*3+1] = bitany; bitans[i1*3+2] = bitanz;
                bitans[i2*3+0] = bitanx; bitans[i2*3+1] = bitany; bitans[i2*3+2] = bitanz;
                bitans[i3*3+0] = bitanx; bitans[i3*3+1] = bitany; bitans[i3*3+2] = bitanz;
            }

            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal:    if i*3+2 >= n.len() {vec3(0.,0.,0.)} else {vec3(n[i*3], n[i*3+1], n[i*3+2])},
                    tex_coords: if i*2+1 >= t.len() {vec2(0.,0.)} else{vec2(t[i*2], t[i*2+1])},
                    tangent:    vec3(tans[i*3], tans[i*3+1], tans[i*3+2]).normalize(),
                    bitangent:  vec3(bitans[i*3], bitans[i*3+1], bitans[i*3+2]).normalize()
                    // ..Vertex::default()
                })
            }

            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture = self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.load_material_texture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.load_material_texture(&material.normal_texture, "texture_normal");
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
            path: path.into()
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
    gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
        0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
}
