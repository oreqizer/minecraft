use std::sync::Arc;
use vulkano::device::{Device as VulkanDevice, DeviceExtensions, Features, QueuesIter, Queue};
use vulkano::instance::{Instance, PhysicalDevice};

use crate::gfx::vulkan::Surface;

pub struct Device {
    pub device: Arc<VulkanDevice>,
    pub queue: Arc<Queue>,
}

impl Device {
    pub fn new(physical: PhysicalDevice, surface: &Surface) -> Self {
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && surface.surface.is_supported(q).unwrap_or(false))
            .expect("couldn't find a Vulkan graphical queue family");

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (device, mut queues) = VulkanDevice::new(
            physical,
            &Features::none(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create Vulkan device");

        // Only a single queue for now
        let queue = queues.next().unwrap();

        Self { device, queue }
    }
}
