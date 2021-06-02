use std::sync::Arc;
use vulkano::device::{Device as VulkanDevice, DeviceExtensions, Features, QueuesIter};
use vulkano::instance::{Instance, PhysicalDevice};

use crate::gfx::vulkan::Surface;

pub struct Device {
    pub device: Arc<VulkanDevice>,
    pub queues: QueuesIter,
}

impl Device {
    pub fn new(physical: PhysicalDevice, surface: &Surface) -> Self {
        const QUEUE_PRIO_DEFAULT: f32 = 0.5;

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && surface.surface.is_supported(q).unwrap_or(false))
            .expect("couldn't find a Vulkan graphical queue family");

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (device, queues) = VulkanDevice::new(
            physical,
            &Features::none(),
            &device_ext,
            [(queue_family, QUEUE_PRIO_DEFAULT)].iter().cloned(),
        )
        .expect("failed to create Vulkan device");

        Self { device, queues }
    }
}
