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
}

impl UiLoader {
    pub fn new() -> Self {
        Self {
            watch_paths: Vec::new(),
            watcher: None,
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<String> {
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        // 添加到监视列表
        self.watch_paths.push(path);
        
        Ok(content)
    }

    pub fn start_watching<F>(&mut self, callback: F) 
    where
        F: Fn() + Send + 'static 
    {
        let (tx, rx) = channel();
        let mut watcher = recommended_watcher(tx).unwrap();

        for path in &self.watch_paths {
            watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        }

        self.watcher = Some(watcher);

        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        println!("File changed: {:?}", event);
                        callback();
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
        });
    }
}