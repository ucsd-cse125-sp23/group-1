#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::{fs, ptr, str};
use gl::types::*;
use cgmath::{Matrix4, Matrix, Vector3, Array, Vector4};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // read shader files
        let v_shader_code = fs::read_to_string(vertex_path)
            .expect("Failed to read vertex shader file");
        let f_shader_code = fs::read_to_string(fragment_path)
            .expect("Failed to read fragment shader file");

        // convert the shader code from String to CString
        let v_shader_code = CString::new(v_shader_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(f_shader_code.as_bytes()).unwrap();

        // compile vertex shader
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }
        shader
    }

    // activate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    // utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector4(&self, name: &CStr, value: &Vector4<f32>) {
        gl::Uniform4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }

    // utility function for checking shader compilation/linking errors
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}