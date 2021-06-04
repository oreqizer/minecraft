use std::sync::Arc;

use vulkano::image::{SwapchainImage};
use vulkano::device::DeviceOwned;
use vulkano::render_pass::{Framebuffer, FramebufferAbstract, RenderPass as VulkanRenderPass};
use winit::window::Window;

use crate::gfx::vulkan::swapchain::Swapchain;
use vulkano::image::view::ImageView;

pub struct RenderPass {
    pub render_pass: Arc<VulkanRenderPass>,
    pub framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
}

impl RenderPass {
    pub fn new(swapchain: &Swapchain) -> Self {
        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                swapchain.swapchain.device().clone(),
                attachments: {
                    // 'color' is a custom name
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let framebuffers = Self::make_framebuffers(&render_pass, &swapchain.images);

        Self {
            render_pass,
            framebuffers,
        }
    }

    fn make_framebuffers(
        render_pass: &Arc<VulkanRenderPass>,
        images: &[Arc<SwapchainImage<Window>>],
    ) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
        images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(ImageView::new(image.clone()).unwrap())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<dyn FramebufferAbstract + Send + Sync>
            })
            .collect::<Vec<_>>()
    }

    pub fn recreate_framebuffers(&mut self, images: &[Arc<SwapchainImage<Window>>]) {
        self.framebuffers = RenderPass::make_framebuffers(&self.render_pass, images);
    }
}
