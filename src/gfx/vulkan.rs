mod device;
mod surface;
mod swapchain;

use std::sync::Arc;

use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::Device as VulkanDevice;
use vulkano::swapchain::Swapchain as VulkanSwapchain;
use winit::{event_loop::EventLoop, window::Window};

pub use device::Device;
pub use surface::Surface;
pub use swapchain::Swapchain;

pub struct Vulkan {
    surface: Surface,
    device: Device,
    swapchain: Swapchain,
}

impl Vulkan {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
            .expect("failed to create Vulkan instance");

        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no Vulkan physical device available");

        let surface = Surface::new(&instance, event_loop);
        let device = Device::new(physical, &surface);
        let swapchain = Swapchain::new(&device, &surface);

        Self {
            surface,
            device,
            swapchain,
        }
    }

    pub fn device(&mut self) -> &Arc<VulkanDevice> {
        &self.device.device
    }

    pub fn swapchain(&mut self) -> &Arc<VulkanSwapchain<Window>> {
        &self.swapchain.swapchain
    }
}
