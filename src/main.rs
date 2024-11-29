use crate::gui::app::App;
use iced::{Application, Settings};

mod gui;
mod screenshare;
mod controller;
mod socket;
mod utils;
mod model;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
