use std::sync::Arc;
use std::time::Duration;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;
use vulkano::swapchain::AcquireError;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::{swapchain, sync};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::game::Input;
use crate::gfx::{events, Vulkan, Window};
use crate::shaders::block;

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
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.vulkan.mark_stale();
                }
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

    fn update(&self, time: Duration, inputs: &[Input]) {
        println!("time: {:?}, update: {:?}", time, inputs);
    }

    fn render(&mut self) {
        self.vulkan.prepare_render();

        // TODO this is just a triangle for now for testing purposes

        // TODO These structs with data will be 'Vertex', 'Block', 'Chunk'
        #[derive(Default, Debug, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        vulkano::impl_vertex!(Vertex, position);

        // TODO these will be created per-chunk
        let vertex_buffer = {
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

        // TODO These will be co-located with shaders in their specific structs
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(self.vulkan.render_pass().clone(), 0).unwrap())
                .build(self.vulkan.device().clone())
                .unwrap(),
        );

        // === ACTUAL RENDER ===
        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.vulkan.swapchain().clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.vulkan.mark_stale();
                    return;
                }
                Err(e) => panic!("Failed to acquire next Vulkan image: {:?}", e),
            };

        if suboptimal {
            self.vulkan.mark_stale();
        }

        // TODO this is temporarily here, make it black and colocate it somewhere
        let clear_values = vec![[0.5, 0.5, 1.0, 1.0].into()];

        let mut builder = AutoCommandBufferBuilder::primary(
            self.vulkan.device().clone(),
            self.vulkan.queue().family(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                self.vulkan.framebuffer(image_num).clone(),
                SubpassContents::Inline,
                clear_values,
            )
            .unwrap()
            .draw(
                pipeline.clone(),
                self.vulkan.dynamic_state(),
                vertex_buffer.clone(),
                (),
                (),
                vec![],
            )
            .unwrap()
            .end_render_pass()
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = self
            .vulkan
            .prev_frame()
            .join(acquire_future)
            .then_execute(self.vulkan.queue().clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.vulkan.queue().clone(),
                self.vulkan.swapchain().clone(),
                image_num,
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.vulkan.replace_frame(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.vulkan.mark_stale();
                self.vulkan
                    .replace_frame(sync::now(self.vulkan.device().clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush Vulkan future: {:?}", e);
                self.vulkan
                    .replace_frame(sync::now(self.vulkan.device().clone()).boxed());
            }
        }
    }
}
