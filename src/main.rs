#![windows_subsystem = "windows"]
mod state;
mod msg;
mod tools;

use state::Display;
use tools::*;

fn main() -> iced::Result {
    Display::default().start()
}