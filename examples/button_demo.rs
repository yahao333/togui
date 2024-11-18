use togui::{Window, Button};

fn main() {
    let window = Window::new("ToGUI Button Demo", 800, 600);
    let _button = Button::new(100.0, 100.0, 200.0, 50.0, "Click Me!");
    window.run();
}