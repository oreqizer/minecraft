use crate::gfx::{Window, Vulkan};

pub struct App {
    window: Window,
    #[allow(dead_code)]
    vulkan: Vulkan,
}

impl App {
    pub fn new() -> Self {
        let vulkan = Vulkan::new();

        Self {
            vulkan: Vulkan::new(),
            window: Window::new(&vulkan),
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
