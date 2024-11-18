use togui::{Window, Button};

fn main() {
    let mut window = Window::new("ToGUI Button Demo", 800, 600);
    let button = Button::new(100.0, 100.0, 200.0, 50.0, "Click Me!");
    window.add_widget(button);
    window.run();
}