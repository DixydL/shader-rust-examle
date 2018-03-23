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
use glutin:: {VirtualKeyCode};


fn main() {
    let (width,height,fear,near) = (1028f32,600f32,100f32,0.1f32);
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

        gl::UseProgram(shader_program);
        gl::BindFragDataLocation(shader_program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        gl::VertexAttribPointer(0, 2, gl::FLOAT,
                                gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        let ortho :GLint = gl::GetUniformLocation(shader_program, CString::new("ortho").unwrap().as_ptr());

        let matrix = Matrix::orthographic(width/2f32,-width/2f32,height/2f32,-height/2f32,fear,0.1);
        let mat_ortho = matrix.get_matrix();


        gl::UniformMatrix4fv(ortho, 1, gl::FALSE, mat_ortho.as_ptr() as *const f32);

        println!("{:?}",mat_ortho);

    }
    let mut fps = calculate::Fps::new(calculate::get_current_time());
    let mut i = 0.;
    let mut status = false;
    let mut status_plus = true;
    let mut running = true;
    let mut key_pressed  = "";
    while running {

        if key_pressed == "W" {
            i+= 1.;
        }
        if key_pressed == "S" {
            i-= 1.;
        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),

                    glutin::WindowEvent::KeyboardInput {input, ..} => match input.virtual_keycode.unwrap() {
                        VirtualKeyCode::W =>  key_pressed = "W",
                        VirtualKeyCode::S =>  key_pressed = "S",
                        VirtualKeyCode::D =>  key_pressed = "D",
                        VirtualKeyCode::A =>  key_pressed = "A",
                        _ => println!("{:?}",input)

                    },
                    _ => ()
                },
                _ => ()
            }




        });


        unsafe {
            let model :GLint = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
            gl::BindVertexArray(VAO);
            println!("{}",key_pressed);
            let matrix = Matrix::translate(0.0,0.0+i,0.);
            let mat = matrix.get_matrix();

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UniformMatrix4fv(model, 1, gl::FALSE, mat.as_ptr() as *const f32);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        //println!("{:?}",duration.as_secs());
        gl_window.swap_buffers().unwrap();
       // fps.show_fps();

    }
}

