//! # Window
//!
//! The `window` module takes care about handling all window-related side effects
//! such as user inputs, IO, updating and rendering. The API is made so that it
//! is **replayable** — replaying the sequence provided by the `on_update` callback
//! always yields the same results.

use std::time::{Duration, SystemTime};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::game::input::Input;
use crate::gfx::events;

pub struct Window {
    // Core
    event_loop: Option<EventLoop<()>>,
    #[allow(dead_code)]
    window: WinitWindow,
    // Callbacks
    on_update: Option<Box<dyn Fn(Duration, &Vec<Input>)>>,
    on_render: Option<Box<dyn Fn()>>,
    // Frames
    curr_time: SystemTime,
    exec_time: Duration,
    // IO
    input_buffer: Vec<Input>,
}

impl Window {
    pub fn new() -> Self {
        const INIT_WIDTH: u32 = 800;
        const INIT_HEIGHT: u32 = 600;
        const TITLE: &str = "Minecraft";

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(TITLE)
            .with_inner_size(LogicalSize::new(
                f64::from(INIT_WIDTH),
                f64::from(INIT_HEIGHT),
            ))
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop: Some(event_loop),
            window,
            //
            on_update: None,
            on_render: None,
            //
            curr_time: SystemTime::now(),
            exec_time: Duration::new(0, 0),
            input_buffer: Vec::new(),
        }
    }

    pub fn on_update(&mut self, f: impl Fn(Duration, &Vec<Input>) + 'static) {
        self.on_update = Some(Box::new(f));
    }

    pub fn on_render(&mut self, f: impl Fn() + 'static) {
        self.on_render = Some(Box::new(f));
    }

    pub fn run(mut self) -> ! {
        let event_loop = self.event_loop.take().unwrap();

        let update = self.on_update.take().unwrap();
        let render = self.on_render.take().unwrap();

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
                        self.input_buffer.push(i);
                    }
                }
                // Device Input events
                Event::DeviceEvent { event, .. } => {
                    if let Some(i) = events::from_device(event) {
                        self.input_buffer.push(i);
                    }
                }
                // Lifecycle
                Event::MainEventsCleared => {
                    const TICK_DUR: Duration = Duration::from_millis(250);

                    self.exec_time += SystemTime::now().duration_since(self.curr_time).unwrap();
                    self.curr_time = SystemTime::now();

                    while self.exec_time > TICK_DUR {
                        update(TICK_DUR, &self.input_buffer);

                        self.exec_time -= TICK_DUR;
                        if self.exec_time <= TICK_DUR {
                            self.input_buffer.clear();
                        }
                    }

                    render();
                }
                // ...rest
                _ => (),
            }
        });
    }
}
