use iced::{Application, Settings};
use crate::gui::app::App;

mod gui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
