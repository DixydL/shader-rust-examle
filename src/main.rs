extern crate gl;
extern crate glutin;


use std::ffi::{CString, CStr};

use std::mem;
use gl::types::*;
use std::ptr;
use glutin::GlContext;
mod calculate {
    use std::time::{Duration,SystemTime,UNIX_EPOCH};
    pub fn get_current_time() -> i32{
       let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i32;
        time
    }
    pub struct Fps{
        i: i32,
        time_now: i32,
        time_before: i32,
        fps: f32,
    }

    impl Fps {
        pub fn new(time_now :i32) -> Fps {
            Fps {
                i : 0,
                time_now : 0,
                time_before : get_current_time(),
                fps: 0 as f32,
            }
        }
        fn fps_calculate(&mut self) -> f32 {
            self.time_now = get_current_time();
            self.i += 1;
            let different = self.time_now - self.time_before;
            if different >= 1 {
                self.time_before = get_current_time();
                self.fps = self.i as f32;
                self.i = 0;
            }

            self.fps
        }

        pub fn show_fps(&mut self) {
            println!("{}",self.fps_calculate())
        }

    }
}

mod matrix {
    struct Matrix {

    }
    impl Matrix {

    }
}
mod shader_mod {

    pub static SHADER_VERTEX: &'static str =
    "
        #version 330
        layout (location = 0) in vec2 position;
        uniform mat4 model;
        void main() {
            gl_Position = mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
            ) * vec4(position.x,position.y, 1.0, 1.0);
        }
    "
    ;

    pub static SHADER_FRAGMENT: &'static str =
        "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";
    pub struct Shader<'a> {
        shader: &'a str
    }

    impl<'a> Shader<'a> {
       pub fn new(shader: &str) -> Shader {
            Shader {
                shader,
            }
        }
        pub fn load(&self) {
            println!("{}", self.shader);
        }
    }

    fn shader_load() {}
}

fn main() {

    let shader = shader_mod::Shader::new(shader_mod::SHADER_VERTEX);
    shader.load();
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(600, 600);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let mut VBO : GLuint= 0;
    let mut VAO : GLuint = 0;
    let mut shader_program: GLuint = 0;

    let mat: [f32;16] =
        [
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0
        ];
    unsafe {
        let vertex_shader : GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let vShaderCode = CString::new(shader_mod::SHADER_VERTEX.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vShaderCode.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader : GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fShaderCode = CString::new(shader_mod::SHADER_FRAGMENT.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fShaderCode.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

       // gl::DeleteShader(vertex_shader);
       // gl::DeleteShader(fragment_shader);

        let vertices :[GLfloat; 6] =
        [
             0.1, 0.1,
             0.3, -0.1, //left
            -0.1, -0.1 //right
        ];
        gl::Viewport(0, 0, 600, 600);

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,(6*std::mem::size_of::<GLfloat>()) as GLsizeiptr, std::mem::transmute(&vertices[0]), gl::DYNAMIC_DRAW);

        gl::UseProgram(shader_program);
        gl::BindFragDataLocation(shader_program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        gl::VertexAttribPointer(0, 2, gl::FLOAT,
                                gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);
    }
    let mut fps = calculate::Fps::new(calculate::get_current_time());
    let mut i = 0;
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            let model :GLint = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
            gl::BindVertexArray(VAO);
            gl::UniformMatrix4fv(model, 1, gl::FALSE, mat.as_ptr());
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        //println!("{:?}",duration.as_secs());
        gl_window.swap_buffers().unwrap();
        fps.show_fps();

    }
}

