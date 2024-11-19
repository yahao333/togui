use notify::{Watcher, RecursiveMode, Result, Event};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

fn main() -> Result<()> {
    // 创建通道用于线程间通信
    let (tx, rx) = channel();
    
    // 创建一个 watcher
    let mut watcher = notify::recommended_watcher(move |result: Result<Event>| {
        match result {
            Ok(event) => tx.send(event).unwrap(),
            Err(e) => println!("监控错误: {:?}", e),
        }
    })?;

    // 要监控的目录路径
    let watch_path = Path::new(".");
    
    // 开始监控目录
    watcher.watch(watch_path, RecursiveMode::Recursive)?;
    
    // 在新线程中处理文件系统事件
    let handle = thread::spawn(move || {
        handle_fs_events(rx);
    });
    
    // 保持主线程运行
    handle.join().unwrap();
    
    Ok(())
}

fn handle_fs_events(rx: Receiver<Event>) {
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
                    mpsc::RecvTimeoutError::Timeout => continue,
                    mpsc::RecvTimeoutError::Disconnected => {
                        println!("通道已断开: {:?}", e);
                        break;
                    }
                }
            }
        }
    }
}