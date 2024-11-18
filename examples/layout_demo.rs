use togui::{Window, Button, Container, Direction, Padding};

fn main() {
    let mut window = Window::new("ToGUI Layout Demo", 800, 600);
    
    let mut container = Container::new(50.0, 50.0, 700.0, 500.0)
        .with_direction(Direction::Vertical)
        .with_padding(Padding::all(20.0))
        .with_spacing(10.0);

    let button1 = Button::new(0.0, 0.0, 200.0, 50.0, "Button 1")
        .on_click(|| println!("Button 1 clicked!"));
    
    let button2 = Button::new(0.0, 0.0, 200.0, 50.0, "Button 2")
        .on_click(|| println!("Button 2 clicked!"));

    container.add_child(button1);
    container.add_child(button2);
    
    window.add_widget(container);
    window.run();
}