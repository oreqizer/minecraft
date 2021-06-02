use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::swapchain::Surface as VulkanSurface;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct Surface {
    pub surface: Arc<VulkanSurface<Window>>,
}

impl Surface {
    pub fn new<T>(instance: &Arc<Instance>, event_loop: &EventLoop<T>) -> Self {
        const INIT_WIDTH: u32 = 800;
        const INIT_HEIGHT: u32 = 600;
        const TITLE: &str = "Minecraft";

        let surface = WindowBuilder::new()
            .with_title(TITLE)
            .with_inner_size(LogicalSize::new(
                f64::from(INIT_WIDTH),
                f64::from(INIT_HEIGHT),
            ))
            .build_vk_surface(event_loop, instance.clone())
            .unwrap();

        Self { surface }
    }
}
