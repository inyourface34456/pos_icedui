#![windows_subsystem = "windows"]
mod display;

use display::Display;

fn main() -> iced::Result {
    Display::default().start()
}