#![windows_subsystem = "windows"]
mod display;
mod item;

use display::Display;

fn main() -> iced::Result {
    Display::default().start()
}