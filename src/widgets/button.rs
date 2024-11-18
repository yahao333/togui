use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::font::Font;

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
    is_hovered: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            is_hovered: false,
        }
    }
}

impl Widget for Button {
    fn draw(&self, renderer: &mut Renderer) {
        // 绘制按钮背景
        let color = if self.is_hovered {
            [100, 100, 100, 255]
        } else {
            [80, 80, 80, 255]
        };
        
        renderer.draw_rect(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
            color,
        );

        // 绘制按钮文本
        let font = Font::default();
        let text_x = self.x as i32 + (self.width as i32 - (self.label.len() * 8) as i32) / 2;
        let text_y = self.y as i32 + (self.height as i32 - 8) / 2;

        for (i, c) in self.label.chars().enumerate() {
            font.render_char(
                renderer,
                text_x + (i * 8) as i32,
                text_y,
                c,
                [255, 255, 255, 255]
            );
        }        
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as f32;
                let y = position.y as f32;
                
                self.is_hovered = x >= self.x 
                    && x <= self.x + self.width
                    && y >= self.y 
                    && y <= self.y + self.height;
            }
            _ => {}
        }
    }
}