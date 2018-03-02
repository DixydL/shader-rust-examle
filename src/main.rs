extern crate gl;
extern crate glutin;
use glutin::GlContext;
#[derive(Debug)]

struct Shader<'a> {
    shader: &'a str
}

impl<'a> Shader<'a> {
    fn mew (shader : &str) -> Shader{
        Shader {
            shader,
        }
    }
    fn load (&self) {
        println!("{}",self.shader);
    }
}
fn shader_load (){

}

fn main() {
    let shader = Shader::mew("xz");
    shader.load();
}

