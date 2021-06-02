use crate::gfx::{Window, Vulkan};

pub struct App {
    window: Window,
    #[allow(dead_code)]
    vulkan: Vulkan,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
            vulkan: Vulkan::new(),
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
