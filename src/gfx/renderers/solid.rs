use std::sync::Arc;

use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::device::Device;
use vulkano::render_pass::{RenderPass, Subpass};

use crate::shaders::block;

#[derive(Default, Debug, Clone)]
struct Vertex {
    position: (f64, f64, f64)
}

vulkano::impl_vertex!(Vertex, position);

struct Renderer {
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
}

impl Renderer {
    pub fn new(device: &Arc<Device>, render_pass: &Arc<RenderPass>) -> Self {
        let vs = block::vs::Shader::load(device.clone()).unwrap();
        let fs = block::fs::Shader::load(device.clone()).unwrap();

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        Self { pipeline }
    }
}
