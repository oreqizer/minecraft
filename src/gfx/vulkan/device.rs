use std::sync::Arc;
use vulkano::device::{Device as VulkanDevice, DeviceExtensions, Features, QueuesIter};
use vulkano::instance::{Instance, PhysicalDevice};

pub struct Device {
    pub device: Arc<VulkanDevice>,
    pub queues: QueuesIter,
}

impl Device {
    pub fn new(instance: &Arc<Instance>) -> Self {
        const QUEUE_PRIO_DEFAULT: f32 = 0.5;

        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no Vulkan physical device available");

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics())
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
