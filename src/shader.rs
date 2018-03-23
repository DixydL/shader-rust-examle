pub mod shader_mod {
    extern crate gl;
    use std::ptr;
    use gl::types::*;
    use std::ffi::{CString, CStr};


    pub static SHADER_VERTEX: &'static str =
        "
        #version 330
        layout (location = 0) in vec2 position;
        uniform mat4 model;
        uniform mat4 ortho;

        void main() {
             gl_Position = ortho * model * vec4(position.x,position.y, 1.0, 1.0);
        }
    "
    ;

    pub static SHADER_FRAGMENT: &'static str =
        "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

    pub const VERTEX :&str =  "vertex";
    pub const FRAGMENT :&str = "fragment";

    pub struct Shader<'a> {
        shader: &'a CString
    }

    pub struct Program {
        program : GLuint
    }

    impl Program {
        pub fn gl_use_program(&self){
            unsafe {
                gl::UseProgram(self.program);
            }
        }

        pub fn get_program(&self) -> GLuint {
            self.program
        }

        pub fn uniform_matrix(&self,name : &str, mat: Vec<f32>) {
            unsafe {
                let model: GLint = gl::GetUniformLocation(self.get_program(), CString::new(name).unwrap().as_ptr());
                gl::UniformMatrix4fv(model, 1, gl::FALSE, mat.as_ptr() as *const f32);
            }

        }
    }

    impl<'a> Shader<'a> {
        pub fn new(shader: &CString) -> Shader {
            Shader {
                shader,
            }
        }
        pub fn load(&self) {


        }

        pub fn compile(&self, types: &str) -> GLuint{
            let mut shader: GLuint = 0;
            let type_shader = if types  == VERTEX { gl::VERTEX_SHADER } else {gl::FRAGMENT_SHADER};
                unsafe {
                shader = gl::CreateShader(type_shader);
                gl::ShaderSource(shader, 1, &self.shader.as_ptr(), ptr::null());
                gl::CompileShader(shader);
            }
            shader
        }

        pub fn attach_shader (vertex: &GLuint, fragment : &GLuint) -> Program{
            let mut shader = Program {
                program : unsafe{gl::CreateProgram()}
            };
            unsafe {
                gl::AttachShader(shader.program, *vertex);
                gl::AttachShader(shader.program, *fragment);
                gl::LinkProgram(shader.program);
            }
            shader
        }

    }

    fn shader_load() {}
}