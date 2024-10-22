use crate::gui::app::App;
use iced::{Application, Settings};

mod gui;
mod capture;
mod screenshare;
mod controller;
mod socket;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
