use std::sync::Arc;
use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::font::Font;
use crate::layout::Rect;

pub struct Button {
    rect: Rect,
    label: String,
    is_hovered: bool,
    is_pressed: bool,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            rect: Rect { x, y, width, height },
            label: label.to_string(),
            is_hovered: false,
            is_pressed: false,
            on_click: None,
        }
    }
    pub fn on_click<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
        self
    }    
}

impl Widget for Button {
    fn draw(&self, renderer: &mut Renderer) {
        // 绘制按钮背景
        let color = if self.is_pressed {
            [60, 60, 60, 255]  // 按下状态
        } else if self.is_hovered {
            [100, 100, 100, 255]  // 悬停状态
        } else {
            [80, 80, 80, 255]  // 普通状态
        };
        
        renderer.draw_rect(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.width as u32,
            self.rect.height as u32,
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
            WindowEvent::MouseInput { 
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if self.is_hovered {
                    self.is_pressed = true;
                }
            }
            WindowEvent::MouseInput { 
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if self.is_pressed && self.is_hovered {
                    if let Some(callback) = &self.on_click {
                        callback();
                    }
                }
                self.is_pressed = false;
            }
            _ => {}
        }
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn preferred_size(&self) -> (f32, f32) {
        (200.0, 50.0)  // 按钮的默认大小
    }    
}