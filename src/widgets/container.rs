use winit::event::WindowEvent;
use super::Widget;
use crate::renderer::Renderer;
use crate::layout::{Rect, Padding, Alignment, Direction};

pub struct Container {
    rect: Rect,
    padding: Padding,
    children: Vec<Box<dyn Widget>>,
    direction: Direction,
    alignment: Alignment,
    spacing: f32,
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

    // 同样的功能 add_child但是名称使用 add_widget
    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
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
        if self.children.is_empty() {
            return;
        }

        // 计算所有子组件的总高度和固定高度
        let mut total_height = 0.0;
        let mut total_flex = 0.0;
        
        for child in &self.children {
            let (_, child_height) = child.preferred_size();
            total_height += child_height;
            total_flex += 1.0; // 未来可以添加 flex 属性
        }

        // 计算间距的总高度
        let spacing_height = self.spacing * (self.children.len() - 1) as f32;
        
        // 计算剩余空间
        let available_height = height - spacing_height;
        let unit_height = if total_flex > 0.0 {
            (available_height - total_height) / total_flex
        } else {
            0.0
        };

        // 布局每个子组件
        let mut current_y = y;
        for child in &mut self.children {
            let (child_preferred_width, child_preferred_height) = child.preferred_size();
            
            // 根据对齐方式计算 x 坐标
            let child_x = match self.alignment {
                Alignment::Start => x,
                Alignment::Center => x + (width - child_preferred_width) / 2.0,
                Alignment::End => x + width - child_preferred_width,
            };

            // 设置子组件的位置和大小
            child.set_rect(Rect {
                x: child_x,
                y: current_y,
                width: child_preferred_width,
                height: child_preferred_height + unit_height,
            });

            current_y += child_preferred_height + unit_height + self.spacing;
        }
    }

    fn layout_horizontal(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if self.children.is_empty() {
            return;
        }

        // 计算所有子组件的总宽度和固定宽度
        let mut total_width = 0.0;
        let mut total_flex = 0.0;
        
        for child in &self.children {
            let (child_width, _) = child.preferred_size();
            total_width += child_width;
            total_flex += 1.0; // 未来可以添加 flex 属性
        }

        // 计算间距的总宽度
        let spacing_width = self.spacing * (self.children.len() - 1) as f32;
        
        // 计算剩余空间
        let available_width = width - spacing_width;
        let unit_width = if total_flex > 0.0 {
            (available_width - total_width) / total_flex
        } else {
            0.0
        };

        // 布局每个子组件
        let mut current_x = x;
        for child in &mut self.children {
            let (child_preferred_width, child_preferred_height) = child.preferred_size();
            
            // 根据对齐方式计算 y 坐标
            let child_y = match self.alignment {
                Alignment::Start => y,
                Alignment::Center => y + (height - child_preferred_height) / 2.0,
                Alignment::End => y + height - child_preferred_height,
            };

            // 设置子组件的位置和大小
            child.set_rect(Rect {
                x: current_x,
                y: child_y,
                width: child_preferred_width + unit_width,
                height: child_preferred_height,
            });

            current_x += child_preferred_width + unit_width + self.spacing;
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
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.layout();  // 重新布局子组件
    }

    fn preferred_size(&self) -> (f32, f32) {
        match self.direction {
            Direction::Horizontal => {
                let mut width: f32 = 0.0;
                let mut max_height: f32 = 0.0;
                for child in &self.children {
                    let (child_width, child_height) = child.preferred_size();
                    width += child_width;
                    max_height = max_height.max(child_height);
                }
                (width, max_height)
            }
            Direction::Vertical => {
                let mut max_width: f32 = 0.0;
                let mut height: f32 = 0.0;
                for child in &self.children {
                    let (child_width, child_height) = child.preferred_size();
                    max_width = max_width.max(child_width);
                    height += child_height;
                }
                (max_width, height)
            }
        }
    }    
}