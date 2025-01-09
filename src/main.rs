use crate::gui::app::App;
use iced::{Font, Pixels, Settings, Size, window};
use iced::multi_window::Application;
use iced::window::{Level, Position};
use iced::window::settings::PlatformSpecific;

mod gui;
mod screenshare;
mod controller;
mod socket;
mod utils;
mod model;


#[tokio::main(flavor = "multi_thread", worker_threads = 8)] // 8 thread
pub async fn main() -> iced::Result {
    App::run(Settings{
        id: None,
        window: window::Settings{
            size: Size::new(1024.0, 768.0),
            position: Position::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: true,
            level: Level::default(),
            icon: None,
            exit_on_close_request: true,
            platform_specific: PlatformSpecific::default(),
        },
        flags: Default::default(),
        fonts: Vec::new(),
        default_font: Font::default(),
        default_text_size: Pixels(16.0),
        antialiasing: false,
    })
}
