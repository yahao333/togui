use crate::renderer::Renderer;

pub mod button;
pub mod text;

pub trait Widget {
    fn draw(&self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: &winit::event::WindowEvent);
}