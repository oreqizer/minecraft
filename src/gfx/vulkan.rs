mod device;
mod surface;
mod swapchain;

use std::sync::Arc;

use vulkano::instance::{Instance, PhysicalDevice};
use winit::{event_loop::EventLoop, window::Window};

pub use device::Device;
pub use surface::Surface;
pub use swapchain::Swapchain;

pub struct Vulkan {
    instance: Arc<Instance>,
    device: Device,
    surface: Surface,
    swapchain: Swapchain,
}

impl Vulkan {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
            .expect("failed to create Vulkan instance");

        let device = Device::new(&instance);
        let surface = Surface::new(&instance, event_loop);
        let swapchain = Swapchain::new(&device, &surface);

        Self {
            instance,
            surface,
            device,
            swapchain,
        }
    }
}
