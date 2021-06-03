use winit::event_loop::{EventLoop, ControlFlow};

use crate::gfx::{events, Window, Vulkan};
use std::time::{Duration, SystemTime};
use crate::game::Input;
use winit::event::{Event, WindowEvent};

pub struct App {
    vulkan: Vulkan,
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        Self {
            vulkan: Vulkan::new(&event_loop),
            window: Some(Window::new()),
            event_loop: Some(event_loop),
        }
    }

    pub fn run(mut self) -> ! {
        let mut window = self.window.take().unwrap();
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                // Window events
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                // Window Input events
                Event::WindowEvent { event, .. } => {
                    if let Some(i) = events::from_window(event) {
                        window.push_input(i);
                    }
                }
                // Device Input events
                Event::DeviceEvent { event, .. } => {
                    if let Some(i) = events::from_device(event) {
                        window.push_input(i);
                    }
                }
                // Lifecycle
                Event::MainEventsCleared => {
                    window.cycle(|time, inputs| {
                        self.update(time, inputs);
                    });

                    self.render();
                }
                // ...rest
                _ => (),
            }
        });
    }

    fn update(&self, time: Duration, inputs: &Vec<Input>) {
        println!("time: {:?}, update: {:?}", time, inputs);
    }

    fn render(&mut self) {
        // println!("render!! {}", count);
    }

    // TODO resize
}
