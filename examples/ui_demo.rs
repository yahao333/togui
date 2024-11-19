use togui::Window;
use togui::ui::{UiLoader, parse_ui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info_log!("Starting ToGUI UI Demo");
    let mut window = Window::new("ToGUI UI Demo", 800, 600);
    let mut loader = UiLoader::new();
    // 设置事件代理
    debug_log!("Setting up event proxy");
    loader.set_event_proxy(window.get_event_proxy());
    // 加载UI文件
    debug_log!("Loading UI file");
    let content = loader.load("examples/demo.ui").unwrap();
    let container = parse_ui(&content).expect("Failed to parse UI");
    window.add_widget(container);

    // 启动热重载
    debug_log!("Starting hot reload watcher");
    loader.start_watching();

    info_log!("Running main window");
    window.run();
    Ok(())
}