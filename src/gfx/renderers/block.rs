use std::io::Cursor;
use std::sync::Arc;

use ash::{vk, Device};
use ash::version::DeviceV1_0;

use crate::gfx::Vulkan;

pub struct Block {
    device: Arc<Device>,

    render_pass: vk::RenderPass,

    shader_vert: vk::ShaderModule,
    shader_frag: vk::ShaderModule,

    // pipeline: vk::Pipeline,
}

impl Block {
    pub fn new(vulkan: &Vulkan) -> Self {
        let device = vulkan.clone_device();

        unsafe {
            // TODO centralize render passes to 'vulkan', will prolly just need one anyway
            // Render pass
            let renderpass_attachments = [
                vk::AttachmentDescription {
                    format: vulkan.surface_format().format,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    ..Default::default()
                },
                vk::AttachmentDescription {
                    format: vk::Format::D16_UNORM,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    ..Default::default()
                },
            ];
            let color_attachment_refs = [vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            }];
            let depth_attachment_ref = vk::AttachmentReference {
                attachment: 1,
                layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            };
            let dependencies = [vk::SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                ..Default::default()
            }];

            let subpasses = [vk::SubpassDescription::builder()
                .color_attachments(&color_attachment_refs)
                .depth_stencil_attachment(&depth_attachment_ref)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()];

            let renderpass_create_info = vk::RenderPassCreateInfo::builder()
                .attachments(&renderpass_attachments)
                .subpasses(&subpasses)
                .dependencies(&dependencies);

            let render_pass = device
                .create_render_pass(&renderpass_create_info, None)
                .unwrap();

            // === SHADERS ===

            let mut vert_file =
                Cursor::new(&include_bytes!("../../../assets/spv/block.vert.spv")[..]);
            let mut frag_file =
                Cursor::new(&include_bytes!("../../../assets/spv/block.frag.spv")[..]);

            let shader_vert = vulkan.create_shader_module(&mut vert_file);
            let shader_frag = vulkan.create_shader_module(&mut frag_file);

            Self {
                device,

                render_pass,

                shader_vert,
                shader_frag,
            }
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.shader_frag, None);
            self.device.destroy_shader_module(self.shader_vert, None);
        }
    }
}
