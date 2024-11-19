mod window;
mod renderer;
mod widgets;
mod font;
mod layout;

pub mod ui;

pub use window::Window;
pub use widgets::Widget;
pub use widgets::button::Button;
pub use widgets::text::Text;
pub use widgets::container::Container;
pub use layout::{Rect, Padding, Alignment, Direction};


#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! info_log {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
}
