use togui::{
    Window, Button, Container, Text,
    Direction, Padding, Alignment
};

fn main() {
    let mut window = Window::new("ToGUI Layout Demo", 800, 600);
    
    // 创建主容器
    let mut main_container = Container::new(10.0, 10.0, 780.0, 580.0)
        .with_direction(Direction::Vertical)
        .with_padding(Padding::all(20.0))
        .with_spacing(10.0);

    // 添加标题文本
    let title = Text::new(0.0, 0.0, "Layout Demo")
        .with_color([255, 255, 255, 255]);
    main_container.add_child(title);

    // 创建水平按钮容器
    let mut button_container = Container::new(0.0, 0.0, 740.0, 50.0)
        .with_direction(Direction::Horizontal)
        .with_spacing(20.0)
        .with_alignment(Alignment::Center);

    // 添加三个按钮
    let button1 = Button::new(0.0, 0.0, 200.0, 50.0, "Button 1")
        .on_click(|| println!("Button 1 clicked!"));
    let button2 = Button::new(0.0, 0.0, 200.0, 50.0, "Button 2")
        .on_click(|| println!("Button 2 clicked!"));
    let button3 = Button::new(0.0, 0.0, 200.0, 50.0, "Button 3")
        .on_click(|| println!("Button 3 clicked!"));

    button_container.add_child(button1);
    button_container.add_child(button2);
    button_container.add_child(button3);

    // 将按钮容器添加到主容器
    main_container.add_widget(button_container);

    // 添加一些说明文本
    let description = Text::new(0.0, 0.0, "This is a demo of the layout system.")
        .with_color([200, 200, 200, 255]);
    main_container.add_child(description);

    window.add_widget(main_container);
    window.run();
}