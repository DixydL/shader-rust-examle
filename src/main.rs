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
mod shader_mod {

    pub static SHADER_VERTEX: &'static str =
        "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
    gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

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
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let mut VBO : GLuint= 0;
    let mut VAO : GLuint = 0;

    unsafe {
        let vertex_shader : GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let vShaderCode = CString::new(shader_mod::SHADER_VERTEX.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vShaderCode.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader : GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fShaderCode = CString::new(shader_mod::SHADER_FRAGMENT.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fShaderCode.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let shader_program :GLuint = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

       // gl::DeleteShader(vertex_shader);
       // gl::DeleteShader(fragment_shader);

        let vertices :[GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,(6*std::mem::size_of::<GLfloat>()) as GLsizeiptr, std::mem::transmute(&vertices), gl::DYNAMIC_DRAW);

        gl::UseProgram(shader_program);
        gl::BindFragDataLocation(shader_program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(shader_program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        //println!("{:?}",duration.as_secs());
        gl_window.swap_buffers().unwrap();
        fps.show_fps();

    }
}

