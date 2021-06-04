use vulkano::command_buffer::DynamicState;
use vulkano::sync;
use vulkano::sync::GpuFuture;
use vulkano::pipeline::viewport::Viewport;

use crate::gfx::vulkan::swapchain::Swapchain;
use vulkano::device::DeviceOwned;

pub struct State {
    pub stale: bool,
    pub dynamic_state: DynamicState,
    pub prev_frame: Option<Box<dyn GpuFuture>>,
}

impl State {
    pub fn new(swapchain: &Swapchain) -> Self {
        let dynamic_state = DynamicState {
            line_width: None,
            viewports: Some(vec![swapchain.viewport()]),
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        Self {
            stale: false,
            dynamic_state,
            prev_frame: Some(sync::now(swapchain.swapchain.device().clone()).boxed()),
        }
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.dynamic_state.viewports = Some(vec![viewport]);
    }

    pub fn replace_frame(&mut self, frame: Box<dyn GpuFuture>) {
        self.prev_frame = Some(frame.boxed());
    }

    pub fn cleanup_frame(&mut self) {
        self.prev_frame.as_mut().unwrap().cleanup_finished();
    }
}
