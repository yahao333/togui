pub mod parser;

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, recommended_watcher, Result as NotifyResult};
use std::sync::mpsc::channel;
use std::time::Duration;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};
use crate::window::CustomEvent;
use std::string::ParseError;
use crate::debug_log;

use std::sync::mpsc;

pub use parser::parse_ui;

#[derive(Debug)]
pub enum LoaderError {
    IoError(io::Error),
    NotifyError(notify::Error),
    ParseError(ParseError),
}

impl From<io::Error> for LoaderError {
    fn from(err: io::Error) -> Self {
        LoaderError::IoError(err)
    }
}

impl From<notify::Error> for LoaderError {
    fn from(err: notify::Error) -> Self {
        LoaderError::NotifyError(err)
    }
}

impl From<ParseError> for LoaderError {
    fn from(err: ParseError) -> Self {
        LoaderError::ParseError(err)
    }
}

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
        debug_log!("Loading UI file: {:?}", path);

        self.current_path = Some(path.clone());

        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        // 添加到监视列表
        debug_log!("UI file content length: {} bytes", content.len());
        self.watch_paths.push(path);
        
        Ok(content)
    }

    pub fn reload(&self) -> io::Result<()> {
        if let Some(path) = &self.current_path {
            let content = std::fs::read_to_string(path)?;
            
            if let Some(proxy) = &self.event_proxy {
                proxy.send_event(CustomEvent::Reload(content))
                    .expect("Failed to send reload event");
            }
        }
        Ok(())
    }

    pub fn start_watching(&mut self) -> Result<(), LoaderError> {
        debug_log!("Starting UI file watcher");
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx).map_err(LoaderError::NotifyError)?;
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

        for path in &self.watch_paths {
            debug_log!("Watching path: {:?}", path);
            watcher.watch(path, RecursiveMode::NonRecursive)
                .map_err(LoaderError::NotifyError)?;
        }

        let event_proxy = self.event_proxy.clone();
        let path = self.watch_paths[0].clone();
        
        // std::thread::spawn(move || {
        //     let mut last_reload = std::time::Instant::now();

        //     loop {
        //         if let Ok(event) = rx.recv() {
        //             debug_log!("Received file system event: {:?}", event);
        //             if let Ok(notify::Event { kind: notify::EventKind::Modify(_), .. }) = event {
        //                 let now = std::time::Instant::now();
        //                 let duration = now.duration_since(last_reload).as_millis();
        //                 debug_log!("Time since last reload: {}ms", duration);                        
        //                 // 确保两次重载之间至少间隔 100ms
        //                 if duration > 200 {
        //                     debug_log!("Attempting to reload file");
        //                     if let Some(proxy) = &event_proxy {
        //                         std::thread::sleep(Duration::from_millis(100));
        //                         match std::fs::read_to_string(&path) {
        //                             Ok(content) => {
        //                                 debug_log!("Successfully read file, content length: {}", content.len());
        //                                 match proxy.send_event(CustomEvent::Reload(content)) {
        //                                     Ok(_) => {
        //                                         debug_log!("Successfully sent reload event");
        //                                         last_reload = now;
        //                                     },
        //                                     Err(e) => debug_log!("Failed to send reload event: {:?}", e),
        //                                 }
        //                             }
        //                             Err(e) => debug_log!("Failed to read UI file: {}", e),
        //                         }
        //                     }else {
        //                         debug_log!("Event proxy is None");
        //                     }
        //                 }else {
        //                     debug_log!("Skipping reload due to time threshold");
        //                 }
        //             }else {
        //                 debug_log!("Event is not a Modify event");
        //             }
        //         }
        //     }
        // });
        // Block forever, printing out events as they come in
        for res in rx {
            match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        self.watcher = Some(watcher);
        Ok(())
    }
}