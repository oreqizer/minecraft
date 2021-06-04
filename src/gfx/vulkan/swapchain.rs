use std::sync::Arc;

use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{ColorSpace, PresentMode, SurfaceTransform, Swapchain as VulkanSwapchain};
use vulkano::sync::SharingMode;
use winit::window::Window;

use crate::gfx::vulkan::device::Device;
use crate::gfx::vulkan::surface::Surface;

pub struct Swapchain {
    pub swapchain: Arc<VulkanSwapchain<Window>>,
    pub images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Swapchain {
    pub fn new(device: &Device, surface: &Surface) -> Self {
        let caps = surface
            .surface
            .capabilities(device.device.physical_device())
            .expect("failed to get Vulkan surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        let (swapchain, images) =
            VulkanSwapchain::start(device.device.clone(), surface.surface.clone())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .layers(1)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(SharingMode::Exclusive)
                .transform(SurfaceTransform::Identity)
                .composite_alpha(alpha)
                .present_mode(PresentMode::Fifo)
                .clipped(true)
                .color_space(ColorSpace::SrgbNonLinear)
                .build()
                .expect("creating Vulkan swapchain failed");

        Self { swapchain, images }
    }

    pub fn recreate_swapchain(&mut self, dimensions: [u32; 2]) {
        let (new_swapchain, new_images) = self
            .swapchain
            .recreate()
            .dimensions(dimensions)
            .build()
            .expect("recreating Vulkan swapchain failed");

        self.swapchain = new_swapchain;
        self.images = new_images;
    }

    pub fn viewport(&self) -> Viewport {
        let dimensions = self.swapchain.dimensions();

        Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        }
    }
}
