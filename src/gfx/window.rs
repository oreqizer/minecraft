use std::sync::Arc;
use std::time::{Duration, SystemTime};

use vulkano_win::VkSurfaceBuild;
use vulkano::swapchain::Surface;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::game::Input;
use crate::gfx::{events, Vulkan};

const TICK_DUR: Duration = Duration::from_millis(250);

pub struct Window {
    curr_time: SystemTime,
    exec_time: Duration,
    input_buffer: Vec<Input>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            curr_time: SystemTime::now(),
            exec_time: Duration::new(0, 0),
            input_buffer: Vec::new(),
        }
    }

    pub fn push_input(&mut self, input: Input) {
        self.input_buffer.push(input);
    }

    pub fn cycle(&mut self, update: impl Fn(Duration, &Vec<Input>)) {
        self.exec_time += SystemTime::now().duration_since(self.curr_time).unwrap();
        self.curr_time = SystemTime::now();

        if self.exec_time > TICK_DUR {
            while self.exec_time > TICK_DUR {
                update(TICK_DUR, &self.input_buffer);

                self.exec_time -= TICK_DUR;
            }
            self.input_buffer.clear();
        }
    }
}
