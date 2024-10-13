use crate::gui::app::App;
use iced::{Application, Settings};

mod gui;
mod capture;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
