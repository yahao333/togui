use super::Widget;

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label: label.to_string(),
        }
    }
}

impl Widget for Button {
    fn draw(&self) {
        // 渲染实现将在后续添加
    }

    fn handle_event(&mut self) {
        // 事件处理将在后续添加
    }
}