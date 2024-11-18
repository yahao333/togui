pub mod parser;

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
pub use parser::parse_ui;

pub struct UiLoader {
    watch_paths: Vec<PathBuf>,
    watcher: Option<notify::RecommendedWatcher>,
    current_path: Option<PathBuf>,
    event_proxy: Option<EventLoopProxy<CustomEvent>>,    
}

impl UiLoader {
    pub fn new() -> Self {
        Self {
            watch_paths: Vec::new(),
            watcher: None,
            current_path: None,
            event_proxy: None,
        }
    }
    pub fn set_event_proxy(&mut self, proxy: EventLoopProxy<CustomEvent>) {
        self.event_proxy = Some(proxy);
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<String> {
        let path = path.as_ref().to_path_buf();
        self.current_path = Some(path.clone());

        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        // 添加到监视列表
        self.watch_paths.push(path);
        
        Ok(content)
    }

    pub fn reload(&self) -> io::Result<()> {
        if let Some(path) = &self.current_path {
            let content = std::fs::read_to_string(path)?;
            let container = parse_ui(&content);
            
            if let Some(proxy) = &self.event_proxy {
                proxy.send_event(CustomEvent::Reload(container))
                    .expect("Failed to send reload event");
            }
        }
        Ok(())
    }

    pub fn start_watching(&mut self) -> io::Result<()> {
        let (tx, rx) = channel();
        let mut watcher = recommended_watcher(tx)?;

        for path in &self.watch_paths {
            watcher.watch(path, RecursiveMode::NonRecursive)?;
        }

        let event_proxy = self.event_proxy.clone();
        
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        match event {
                            notify::Event { kind: notify::EventKind::Modify(_), .. } => {
                                println!("File changed, reloading...");
                                if let Some(proxy) = &event_proxy {
                                    // 等待文件写入完成
                                    std::thread::sleep(Duration::from_millis(100));
                                    let content = std::fs::read_to_string(path)
                                        .expect("Failed to read UI file");
                                    let container = parse_ui(&content);
                                    proxy.send_event(CustomEvent::Reload(container))
                                        .expect("Failed to send reload event");
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
        });

        self.watcher = Some(watcher);
        Ok(())
    }
}