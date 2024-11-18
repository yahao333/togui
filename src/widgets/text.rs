use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::font::Font;
use crate::layout::Rect;

pub struct Text {
    rect: Rect,
    content: String,
    color: [u8; 4],
    font: Font,
}

impl Text {
    pub fn new(x: f32, y: f32, content: &str) -> Self {
        Self {
            rect: Rect {
                x,
                y,
                width: content.len() as f32 * 8.0,  // 假设每个字符宽度为8
                height: 8.0,  // 字体高度为8
            },
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
                self.rect.x as i32 + x_offset,
                self.rect.y as i32,
                c,
                self.color
            );
            x_offset += 8;
        }
    }

    fn handle_event(&mut self, _event: &WindowEvent) {
        // 文本组件不需要处理事件
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn preferred_size(&self) -> (f32, f32) {
        (self.content.len() as f32 * 8.0, 8.0)
    }
}