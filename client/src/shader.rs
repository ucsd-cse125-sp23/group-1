use std::ffi::CString;
use std::{fs, ptr, str};
use gl::types::*;

pub struct Shader {
    pub ID: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { ID: 0 };

        // read shader files
        let v_shader_code = fs::read_to_string("shaders/shader.vs")
            .expect("Failed to read vertex shader file");
        let f_shader_code = fs::read_to_string("shaders/shader.fs")
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
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.check_compile_errors(ID, "PROGRAM");

            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.ID = ID;
        }
        shader
    }

    // activate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.ID);
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