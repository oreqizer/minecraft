use std::sync::Arc;

use vulkano::instance::Instance;

pub struct Vulkan {
    #[allow(dead_code)]
    pub instance: Arc<Instance>,
}

impl Vulkan {
    pub fn new() -> Self {
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
            .expect("failed to create Vulkan instance");

        Self { instance }
    }
}
