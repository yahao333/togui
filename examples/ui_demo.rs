use togui::Window;
use togui::ui::{UiLoader, parse_ui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("ToGUI UI Demo", 800, 600);
    let mut loader = UiLoader::new();
    // 设置事件代理
    loader.set_event_proxy(window.get_event_proxy());
    // 加载UI文件
    let content = loader.load("examples/demo.ui").unwrap();
    let container = parse_ui(&content).expect("Failed to parse UI");
    window.add_widget(container);

    // 启动热重载
    loader.start_watching();

    window.run();
    Ok(())
}