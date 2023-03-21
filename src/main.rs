#![windows_subsystem = "windows"]
use iced::{
    alignment,
    widget::{checkbox::Icon, text, Text},
    window::{self, icon},
    Application, Font, Settings,
};
use state::State;

mod filter;
mod message;
mod password;
mod state;
mod view;

fn main() -> iced::Result {
    let ico = icon::Icon::from_file("icons/image.ico").expect("Missing icon");
    State::run(Settings {
        window: window::Settings {
            size: (800, 250),
            icon: Some(ico),
            ..Default::default()
        },
        ..Default::default()
    })
}
