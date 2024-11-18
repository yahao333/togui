use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::layout::{Rect, Padding, Alignment};

pub struct Container {
    rect: Rect,
    padding: Padding,
    children: Vec<Box<dyn Widget>>,
    direction: Direction,
    alignment: Alignment,
    spacing: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Container {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect { x, y, width, height },
            padding: Padding::all(10.0),
            children: Vec::new(),
            direction: Direction::Vertical,
            alignment: Alignment::Start,
            spacing: 5.0,
        }
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn add_child<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
        self.layout();
    }

    fn layout(&mut self) {
        let content_x = self.rect.x + self.padding.left;
        let content_y = self.rect.y + self.padding.top;
        let content_width = self.rect.width - (self.padding.left + self.padding.right);
        let content_height = self.rect.height - (self.padding.top + self.padding.bottom);

        match self.direction {
            Direction::Vertical => self.layout_vertical(content_x, content_y, content_width, content_height),
            Direction::Horizontal => self.layout_horizontal(content_x, content_y, content_width, content_height),
        }
    }

    fn layout_vertical(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let mut current_y = y;
        
        for child in &mut self.children {
            // TODO: 实现具体的垂直布局逻辑
            current_y += self.spacing;
        }
    }

    fn layout_horizontal(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let mut current_x = x;
        
        for child in &mut self.children {
            // TODO: 实现具体的水平布局逻辑
            current_x += self.spacing;
        }
    }
}

impl Widget for Container {
    fn draw(&self, renderer: &mut Renderer) {
        for child in &self.children {
            child.draw(renderer);
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        for child in &mut self.children {
            child.handle_event(event);
        }
    }
}