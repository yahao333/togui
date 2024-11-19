use crate::renderer::Renderer;
use crate::widgets::Widget;
use crate::Container;
use std::vec::Vec;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window as WinitWindow, WindowBuilder},
};
use crate::ui::parser::parse_ui;

#[derive(Debug, Clone)]
pub enum CustomEvent {
    Reload(String),  // 改为传递 UI 文件内容而不是 Container
}

pub struct Window {
    event_loop: EventLoop<CustomEvent>,
    window: WinitWindow,
    renderer: Renderer,
    widgets: Vec<Box<dyn Widget>>,
    event_proxy: EventLoopProxy<CustomEvent>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let event_proxy = event_loop.create_proxy();

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
            event_proxy,
        }
    }

    pub fn get_event_proxy(&self) -> EventLoopProxy<CustomEvent> {
        self.event_proxy.clone()
    }

    pub fn reload_ui(&mut self, container: Container) {
        self.widgets.clear();
        self.add_widget(container);
    }

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget));
    }

    pub fn run(mut self) {
        let window = self.window;
        let mut widgets = self.widgets;
        let mut renderer = self.renderer;
        let event_loop = self.event_loop;

        let mut frame_count = 0;
        let mut last_time = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == window.id() => {
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
                Event::UserEvent(CustomEvent::Reload(content)) => {
                    debug_log!("Reloading UI with content length: {}", content.len());
                    // 解析新的UI内容
                    match parse_ui(&content) {
                        Ok(container) => {
                            widgets.clear();
                            widgets.push(Box::new(container));
                            window.request_redraw();
                        }
                        Err(e) => {
                            println!("Failed to parse UI: {:?}", e);
                        }
                    }
                }                
                Event::RedrawRequested(_) => {
                    frame_count += 1;
                    let now = std::time::Instant::now();
                    if now.duration_since(last_time).as_secs() >= 1 {
                        debug_log!("FPS: {}", frame_count);
                        frame_count = 0;
                        last_time = now;
                    }

                    renderer.clear([64, 64, 64, 255]);
                    
                    // 绘制所有组件
                    for widget in &widgets {
                        widget.draw(&mut renderer);
                    }
                    
                    // renderer.render().unwrap();
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
}