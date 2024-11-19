use std::io;
use crate::{Container, Button, Text};
use crate::layout::{Direction, Alignment, Padding};

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    IoError(io::Error),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IoError(err)
    }
}

pub fn parse_ui(content: &str) -> Result<Container, ParseError> {
    // 这里先实现一个简单的解析器
    let mut lines = content.lines();
    let mut container = Container::new(0.0, 0.0, 800.0, 600.0);

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("Container") {
            // 解析容器属性
            let _ = parse_container(&mut container, line);
        } else if line.starts_with("Button") {
            // 解析按钮
            if let Some(button) = parse_button(line) {
                container.add_child(button);
            }
        } else if line.starts_with("Text") {
            // 解析文本
            if let Some(text) = parse_text(line) {
                container.add_child(text);
            }
        }
    }

    Ok(container)
}

fn parse_container(container: &mut Container, line: &str) -> Result<(), ParseError> {
    // 简单的属性解析
    if line.contains("vertical") {
        container.with_direction(Direction::Vertical);
    }
    if line.contains("horizontal") {
        container.with_direction(Direction::Horizontal);
    }
    if line.contains("center") {
        container.with_alignment(Alignment::Center);
    }
    Ok(())
}

fn parse_button(line: &str) -> Option<Button> {
    // 简单的按钮解析
    if let Some(label) = line.split('"').nth(1) {
        Some(Button::new(0.0, 0.0, 200.0, 50.0, label))
    } else {
        None
    }
}

fn parse_text(line: &str) -> Option<Text> {
    // 简单的文本解析
    if let Some(content) = line.split('"').nth(1) {
        Some(Text::new(0.0, 0.0, content))
    } else {
        None
    }
}