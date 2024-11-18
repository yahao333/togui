use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;

pub struct Text {
    x: f32,
    y: f32,
    content: String,
}

impl Text {
    pub fn new(x: f32, y: f32, content: &str) -> Self {
        Self {
            x,
            y,
            content: content.to_string(),
        }
    }
}

impl Widget for Text {
    fn draw(&self, renderer: &mut Renderer) {
        // 渲染实现将在后续添加
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        // 事件处理将在后续添加
    }
}