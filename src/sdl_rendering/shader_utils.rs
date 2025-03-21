use gl;
use std::ffi::CString;
use std::fs;
use std::ptr;
use nalgebra::Matrix4;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let vertex_code = fs::read_to_string(vertex_path).unwrap();
        let fragment_code = fs::read_to_string(fragment_path).unwrap();

        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let c_str_vert = CString::new(vertex_code.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let c_str_frag = CString::new(fragment_code.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
        }

        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: program }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_ptr());
        }
    }
}
