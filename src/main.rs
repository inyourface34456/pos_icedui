#![windows_subsystem = "windows"]
mod state;
mod msg;

use state::State;

fn main() -> iced::Result {
    State::default().start()
}