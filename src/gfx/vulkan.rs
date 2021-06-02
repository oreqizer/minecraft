mod device;

use std::sync::Arc;

use vulkano::instance::{Instance, PhysicalDevice};

use device::Device;

pub struct Vulkan {
    pub instance: Arc<Instance>,
    device: Device,
}

impl Vulkan {
    pub fn new() -> Self {
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
            .expect("failed to create Vulkan instance");

        let device = Device::new(&instance);

        Self {
            instance,
            device,
        }
    }
}
