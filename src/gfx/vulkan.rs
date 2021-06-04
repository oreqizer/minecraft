mod device;
mod render_pass;
mod surface;
mod swapchain;
mod state;

use std::sync::Arc;

use vulkano::device::{Device as VulkanDevice, Queue};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::render_pass::{RenderPass as VulkanRenderPass, FramebufferAbstract};
use vulkano::swapchain::Swapchain as VulkanSwapchain;
use vulkano::command_buffer::DynamicState;
use vulkano::sync::GpuFuture;
use winit::{event_loop::EventLoop, window::Window};

use device::Device;
use render_pass::RenderPass;
use surface::Surface;
use swapchain::Swapchain;
use state::State;

pub struct Vulkan {
    surface: Surface,
    device: Device,
    swapchain: Swapchain,
    render_pass: RenderPass,
    state: State,
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
        let render_pass = RenderPass::new(&swapchain);
        let state = State::new(&swapchain);

        Self {
            surface,
            device,
            swapchain,
            render_pass,
            state,
        }
    }

    pub fn device(&self) -> &Arc<VulkanDevice> {
        &self.device.device
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.device.queue
    }

    pub fn swapchain(&self) -> &Arc<VulkanSwapchain<Window>> {
        &self.swapchain.swapchain
    }

    pub fn render_pass(&self) -> &Arc<VulkanRenderPass> {
        &self.render_pass.render_pass
    }

    pub fn framebuffer(&self, image_num: usize) -> &Arc<dyn FramebufferAbstract + Send + Sync> {
        &self.render_pass.framebuffers[image_num]
    }

    pub fn dynamic_state(&self) -> &DynamicState {
        &self.state.dynamic_state
    }

    pub fn prev_frame(&mut self) -> Box<dyn GpuFuture> {
        self.state.prev_frame.take().unwrap()
    }

    pub fn mark_stale(&mut self) {
        self.state.stale = true;
    }

    pub fn prepare_render(&mut self) {
        self.state.cleanup_frame();

        if self.state.stale {
            let dimensions: [u32; 2] = self.surface.surface.window().inner_size().into();

            self.swapchain.recreate_swapchain(dimensions);
            self.state.set_viewport(self.swapchain.viewport());
            self.render_pass
                .recreate_framebuffers(&self.swapchain.images);
        }
    }

    pub fn replace_frame(&mut self, frame: Box<dyn GpuFuture>) {
        self.state.replace_frame(frame);
    }
}
