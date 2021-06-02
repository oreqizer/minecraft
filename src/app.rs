use winit::event_loop::EventLoop;

use crate::gfx::{Window, Vulkan};

pub struct App {
    window: Window,
    vulkan: Vulkan,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        Self {
            vulkan: Vulkan::new(&event_loop),
            window: Window::new(event_loop),
        }
    }

    pub fn run(mut self) -> ! {
        // TODO take necessary environment from self via Option::take and unwrapping
        // and use in closures, such as renderer

        self.window.on_update(|time, inputs| {
            println!("time: {:?}, update: {:?}", time, inputs);
        });

        self.window.on_render(|| {
            // println!("render!!");
        });

        self.window.run();
    }
}
