pub mod button;
pub mod text;

pub trait Widget {
    fn draw(&self);
    fn handle_event(&mut self);
}