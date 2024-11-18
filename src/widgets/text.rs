use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::font::Font;

pub struct Text {
    x: f32,
    y: f32,
    content: String,
    color: [u8; 4],
    font: Font,
}

impl Text {
    pub fn new(x: f32, y: f32, content: &str) -> Self {
        Self {
            x,
            y,
            content: content.to_string(),
            color: [255, 255, 255, 255], // 默认白色
            font: Font::default(),
        }
    }

    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }    
}

impl Widget for Text {
    fn draw(&self, renderer: &mut Renderer) {
        let mut x_offset = 0;
        for c in self.content.chars() {
            self.font.render_char(
                renderer,
                self.x as i32 + x_offset,
                self.y as i32,
                c,
                self.color
            );
            x_offset += 8; // 每个字符宽度为 8 像素
        }
    }

    fn handle_event(&mut self, _event: &WindowEvent) {
        // 文本组件暂时不需要处理事件
    }
}