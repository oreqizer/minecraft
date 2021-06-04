use std::time::{Duration, SystemTime};

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::game::Input;
use crate::gfx::{events, Vulkan, Window};
use crate::shaders::block;
use std::sync::Arc;

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
        // TODO this is just a triangle for now for testing purposes
        let vertex_buffer = {
            #[derive(Default, Debug, Clone)]
            struct Vertex {
                position: [f32; 2],
            }
            vulkano::impl_vertex!(Vertex, position);

            CpuAccessibleBuffer::from_iter(
                self.vulkan.device().clone(),
                BufferUsage::all(),
                false,
                [
                    Vertex {
                        position: [-0.5, -0.25],
                    },
                    Vertex {
                        position: [0.0, 0.5],
                    },
                    Vertex {
                        position: [0.25, -0.1],
                    },
                ]
                .iter()
                .cloned(),
            )
            .unwrap()
        };

        let vs = block::vs::Shader::load(self.vulkan.device().clone()).unwrap();
        let fs = block::fs::Shader::load(self.vulkan.device().clone()).unwrap();

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                self.vulkan.device().clone(),
                attachments: {
                    // Color is a custom name
                    color: {
                        load: Clear,
                        store: Store,
                        format: self.vulkan.swapchain().format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );
    }

    // TODO resize
}
