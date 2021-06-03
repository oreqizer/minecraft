pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "src/shaders/block.vert.glsl",
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "src/shaders/block.frag.glsl",
    }
}
