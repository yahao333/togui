use crate::renderer::Renderer;
use crate::layout::Rect;

pub mod button;
pub mod text;
pub mod container;

pub trait Widget {
    fn draw(&self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: &winit::event::WindowEvent);
    
    // 新增的布局相关方法
    fn get_rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
    fn preferred_size(&self) -> (f32, f32) {
        (0.0, 0.0)  // 默认实现
    }
}