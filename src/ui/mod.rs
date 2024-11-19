pub mod parser;

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, recommended_watcher, Result as NotifyResult, Event as NotifyEvent};
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
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::sync::mpsc::RecvTimeoutError;

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
        // 创建通道用于线程间通信
        let (tx, rx) = channel();

        // 创建一个 watcher
        let mut watcher = notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
            match result {
                Ok(event) => tx.send(event).unwrap(),
                Err(e) => println!("监控错误: {:?}", e),
            }
        })?;

        // for path in &self.watch_paths {
        //     debug_log!("Watching path: {:?}", path);
        //     watcher.watch(path, RecursiveMode::NonRecursive)
        //         .map_err(LoaderError::NotifyError)?;
        // }
        // 要监控的目录路径
        let watch_path = Path::new(".");
        
        // 开始监控目录
        watcher.watch(watch_path, RecursiveMode::NonRecursive)?;

        // 在新线程中处理文件系统事件
        let handle = thread::spawn(move || {
            Self::handle_fs_events(rx);
        });

        /*
        let event_proxy = self.event_proxy.clone();
        let path = self.watch_paths[0].clone();
        
        std::thread::spawn(move || {
            let mut last_reload = std::time::Instant::now();

            loop {
                if let Ok(event) = rx.recv() {
                    debug_log!("Received file system event: {:?}", event);
                    if let Ok(notify::Event { kind: notify::EventKind::Modify(_), .. }) = event {
                        let now = std::time::Instant::now();
                        let duration = now.duration_since(last_reload).as_millis();
                        debug_log!("Time since last reload: {}ms", duration);                        
                        // 确保两次重载之间至少间隔 100ms
                        if duration > 200 {
                            debug_log!("Attempting to reload file");
                            if let Some(proxy) = &event_proxy {
                                std::thread::sleep(Duration::from_millis(100));
                                match std::fs::read_to_string(&path) {
                                    Ok(content) => {
                                        debug_log!("Successfully read file, content length: {}", content.len());
                                        match proxy.send_event(CustomEvent::Reload(content)) {
                                            Ok(_) => {
                                                debug_log!("Successfully sent reload event");
                                                last_reload = now;
                                            },
                                            Err(e) => debug_log!("Failed to send reload event: {:?}", e),
                                        }
                                    }
                                    Err(e) => debug_log!("Failed to read UI file: {}", e),
                                }
                            }else {
                                debug_log!("Event proxy is None");
                            }
                        }else {
                            debug_log!("Skipping reload due to time threshold");
                        }
                    }else {
                        debug_log!("Event is not a Modify event");
                    }
                }
            }
        });
 */
        self.watcher = Some(watcher);
        Ok(())
    }

    fn handle_fs_events(rx: Receiver<NotifyEvent>) {
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Create(_) => println!("创建: {:?}", event.paths),
                        notify::EventKind::Modify(_) => println!("修改: {:?}", event.paths),
                        notify::EventKind::Remove(_) => println!("删除: {:?}", event.paths),
                        _ => println!("其他事件: {:?}", event),
                    }
                }
                Err(e) => {
                    match e {
                        RecvTimeoutError::Timeout => continue,
                        RecvTimeoutError::Disconnected => {
                            println!("通道已断开: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
    }
}