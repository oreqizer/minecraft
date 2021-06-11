use std::io::Cursor;

use ash::vk;
use ash::version::DeviceV1_0;

use crate::gfx::Vulkan;

struct Block {
    shader_vert: vk::ShaderModule,
    shader_frag: vk::ShaderModule,

    // pipeline: vk::Pipeline,
}

impl Block {
    pub fn new(vulkan: &Vulkan) -> Self {
        let mut vert_file =
            Cursor::new(&include_bytes!("../../../assets/spv/block.vert.spv")[..]);
        let mut frag_file =
            Cursor::new(&include_bytes!("../../../assets/spv/block.frag.spv")[..]);

        let shader_vert = vulkan.create_shader_module(&mut vert_file);
        let shader_frag = vulkan.create_shader_module(&mut frag_file);

        Self {
            shader_vert,
            shader_frag,
        }
    }

    pub fn destroy(&mut self, vulkan: &Vulkan) {
        let device = vulkan.device().device();

        unsafe {
            device.destroy_shader_module(self.shader_frag, None);
            device.destroy_shader_module(self.shader_vert, None);
        }
    }
}

