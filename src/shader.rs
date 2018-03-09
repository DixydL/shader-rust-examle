pub mod shader_mod {

    pub static SHADER_VERTEX: &'static str =
        "
        #version 330
        layout (location = 0) in vec2 position;
        uniform mat4 model;
        void main() {
            gl_Position = model * vec4(position.x,position.y, 1.0, 1.0);
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