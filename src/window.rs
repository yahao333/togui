use crate::renderer::Renderer;
use crate::widgets::Widget;
use std::vec::Vec;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    renderer: Renderer,
    widgets: Vec<Box<dyn Widget>>,
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
            widgets: Vec::new(),
        }
    }

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget));
    }

    pub fn run(self) {
        let window_id = self.window.id();
        let mut renderer = self.renderer;
        let window = self.window;
        let mut widgets = self.widgets;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    window_id: event_window_id,
                    ref event,
                } if event_window_id == window_id => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {
                            // 处理组件事件
                            for widget in &mut widgets {
                                widget.handle_event(event);
                            }
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    self.renderer.clear([64, 64, 64, 255]);
                    
                    // 绘制所有组件
                    for widget in &widgets {
                        widget.draw(&mut self.renderer);
                    }
                    
                    self.renderer.render().unwrap();
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => (),
            }
        });
    }
}