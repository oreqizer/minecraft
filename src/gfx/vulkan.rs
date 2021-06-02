use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

pub struct Vulkan {
    #[allow(dead_code)]
    instance: Arc<Instance>,
}

impl Vulkan {
    pub fn new() -> Self {
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("failed to create vulkano::instance");

        Self {
            instance,
        }
    }
}
