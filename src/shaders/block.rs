// TODO this is just a placeholder for testing purposes

pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        // language=GLSL
        src: "
#version 450

layout(location = 0) in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}
",
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        // language=GLSL
        src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
",
    }
}
