use crate::renderer::Renderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    renderer: Renderer,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();
        
        let renderer = Renderer::new(&window);

        Self { 
            event_loop, 
            window,
            renderer,
        }
    }

    pub fn run(self) {
        let window_id = self.window.id();
        let mut renderer = self.renderer;
        let window = self.window;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    window_id: event_window_id,
                    event,
                } if event_window_id == window_id => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    }
                }
                Event::RedrawRequested(_) => {
                    renderer.clear([64, 64, 64, 255]);
                    renderer.render().unwrap();
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
}