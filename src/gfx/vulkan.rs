mod instance;
mod device;
mod swapchain;

use std::io::Cursor;

use ash::util::read_spv;
use ash::vk;
use ash::version::DeviceV1_0;
use winit::{event_loop::EventLoop};

use instance::Instance;
use device::Device;
use swapchain::Swapchain;

pub struct Vulkan {
    instance: Instance,
    device: Device,
    swapchain: Swapchain,
}

impl Vulkan {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        const TITLE: &str = "Minecraft";

        let instance = Instance::new(event_loop);
        let device = Device::new(&instance);
        let swapchain = Swapchain::new(&instance, &device);

        Self {
            instance,
            device,
            swapchain,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn create_shader_module(&self, file: &mut Cursor<&[u8]>) -> vk::ShaderModule {
        let code = read_spv(file).unwrap();

        let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);

        unsafe {
            self
                .device.device()
                .create_shader_module(&create_info, None)
                .unwrap()
        }
    }

    pub fn prepare_render(&mut self) {
        // self.state.cleanup_frame();
        //
        // if self.state.stale {
        //     let dimensions: [u32; 2] = self.surface.surface.window().inner_size().into();
        //
        //     self.swapchain.recreate_swapchain(dimensions);
        //     self.state.set_viewport(self.swapchain.viewport());
        //     self.render_pass
        //         .recreate_framebuffers(&self.swapchain.images);
        // }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        self.swapchain.destroy(&self.device);
        self.device.destroy();
        self.instance.destroy();
    }
}
