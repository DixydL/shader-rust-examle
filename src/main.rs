extern crate gl;
extern crate glutin;
use std::ffi::{CString, CStr};


use gl::types::*;
use std::ptr;
use glutin::GlContext;
mod shader_mod {

   pub const SHADER_VERTEX: &str = "
    #version 330 core
    layout (location = 0) in vec3 position;
    void main()
    {
        gl_Position = vec4(position.x, position.y, position.z, 1.0)
    }
    ";

    pub const  SHADER_FRAGMENT: &str = "
    #version 330 core
    out vec4 color;
    void main()
    {
        color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
    ";
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
    }


    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        let vertex_shader : GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let fShaderCode = CString::new(shader_mod::SHADER_FRAGMENT.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &fShaderCode.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

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
        }

        gl_window.swap_buffers().unwrap();
    }
}

