use std::sync::Arc;

use winit::window::Window;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::sync::SharingMode;
use vulkano::swapchain::{
    ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform,
    Swapchain as VulkanSwapchain,
};

use crate::gfx::vulkan::device::Device;
use crate::gfx::vulkan::surface::Surface;

pub struct Swapchain {
    pub(crate) swapchain: Arc<VulkanSwapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Swapchain {
    pub fn new(device: &Device, surface: &Surface) -> Self {
        let caps = surface.surface
            .capabilities(device.device.physical_device())
            .expect("failed to get Vulkan surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        let (swapchain, images) = VulkanSwapchain::start(device.device.clone(), surface.surface.clone())
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
}
