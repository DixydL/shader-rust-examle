extern crate gl;
extern crate glutin;
extern crate ma_rgl;
mod shader;
mod fps_calculate;

use ma_rgl::matrix::Matrix;
use std::ffi::{CString, CStr};
use std::mem;
use shader::shader_mod;
use fps_calculate::calculate;
use gl::types::*;
use std::ptr;
use glutin::GlContext;
use glutin::VirtualKeyCode;
use std::default::Default;


mod keyboard {
    struct KeyCode {
        pressed: bool,
        key: i32
    }

    impl KeyCode {
            fn new(){

            }
    }
}
fn main() {
    let (width,height,fear,near) = (1028f32,600f32,100f32,0.1f32);
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
    let mut  program : shader_mod::Program;
    unsafe {
        let vShaderCode = CString::new(shader_mod::SHADER_VERTEX.as_bytes()).unwrap();
        let vector = shader::shader_mod::Shader::new(&vShaderCode);
        let vertex_shader = vector.compile(shader_mod::VERTEX);

        let fShaderCode = CString::new(shader_mod::SHADER_FRAGMENT.as_bytes()).unwrap();
        let fragment = shader::shader_mod::Shader::new(&fShaderCode);
        let fragment_shader = fragment.compile(shader_mod::FRAGMENT);

        program = shader_mod::Shader::attach_shader(&vertex_shader,&fragment_shader);
        shader_program = program.get_program();


        let vertices :[GLfloat; 6] =
        [
             10.1, 10.1,
             10.3, -10.1, //left
            -10.1, -10.1 //right
        ];
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,(6*std::mem::size_of::<GLfloat>()) as GLsizeiptr, std::mem::transmute(&vertices[0]), gl::DYNAMIC_DRAW);
        program.gl_use_program();
        gl::BindFragDataLocation(shader_program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        gl::VertexAttribPointer(0, 2, gl::FLOAT,
                                gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        let matrix = Matrix::orthographic(width/2f32,-width/2f32,height/2f32,-height/2f32,fear,0.1);
        let mat_ortho = matrix.get_matrix();

        program.uniform_matrix("ortho",mat_ortho);

    }
    let mut fps = calculate::Fps::new(calculate::get_current_time());
    let mut y = 0.;
    let mut x = 0.;
    let mut status = false;
    let mut status_plus = true;
    let mut running = true;
    let mut grab = false;
    let mut key_pressed  = Default::default();
    while running {

        if key_pressed == "W" && grab {
            y+= 1.;
        }
        if key_pressed == "S" && grab {
            y-= 1.;
        }
        if key_pressed == "A" && grab {
            x-= 1.;
        }
        if key_pressed == "D" && grab {
            x+= 1.;
        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),

                    glutin::WindowEvent::KeyboardInput {input, ..} => match input {
                        glutin::KeyboardInput {state, virtual_keycode, ..} => {
                            if state == glutin::ElementState::Pressed {
                                grab = true;
                            } else {
                                grab = false;
                            }
                            match virtual_keycode.unwrap() {
                                VirtualKeyCode::W => key_pressed = "W",
                                VirtualKeyCode::S => key_pressed = "S",
                                VirtualKeyCode::A => key_pressed = "A",
                                VirtualKeyCode::D => key_pressed = "D",
                                _ => key_pressed = "",
                            }

                        },

                        _ => println!("{:?}", input)
                    },
                    _ => ()
                },
                _ => ()
            }




        });


        unsafe {
            gl::BindVertexArray(VAO);
            println!("{}",key_pressed);
            let matrix = Matrix::translate(0.0+x,0.0+y,0.);
            let mat = matrix.get_matrix();
            program.uniform_matrix("model",mat);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);


            gl::BindVertexArray(0);
        }

        //println!("{:?}",duration.as_secs());
        gl_window.swap_buffers().unwrap();
       // fps.show_fps();

    }
}

