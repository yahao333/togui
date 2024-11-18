use togui::Window;
use togui::ui::{UiLoader, parse_ui};

fn main() {
    let mut window = Window::new("ToGUI UI Demo", 800, 600);
    let mut loader = UiLoader::new();

    // 加载UI文件
    let content = loader.load("examples/demo.ui").unwrap();
    let container = parse_ui(&content);
    window.add_widget(container);

    // 启动热重载
    loader.start_watching(|| {
        println!("UI file changed, reloading...");
        // TODO: 实现重新加载逻辑
    });

    window.run();
}