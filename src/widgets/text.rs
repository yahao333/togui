use super::Widget;

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
    fn draw(&self) {
        // 渲染实现将在后续添加
    }

    fn handle_event(&mut self) {
        // 事件处理将在后续添加
    }
}