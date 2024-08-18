use crate::gui::app::App;
use iced::{Application, Settings};

mod gui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
