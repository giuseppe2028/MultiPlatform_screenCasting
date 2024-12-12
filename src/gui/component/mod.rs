use std::fmt::Debug;
use iced::{Command};

use crate::gui::app;
use crate::gui::theme::widget::Element;

pub mod connection;
pub mod home;
pub mod receiver_ip;
pub mod receiver_streaming;
pub mod caster_settings;
pub mod caster_streaming;
pub mod window_part_screen;
pub mod shorcut;
pub mod keycodeutils;
pub mod AnnotationToolsComponent;
pub(crate) mod Annotation;

pub trait Component<'a> {
    type Message: Into<app::Message> + Clone + Debug;
    //type UpdateProps;
    //type ViewProps;

    fn update(&mut self, message: Self::Message) -> Command<app::Message>;
    fn view(&self /*, props: Self::ViewProps*/) -> Element<'_, app::Message>;
    fn subscription(&self) -> iced::Subscription<Self::Message>;
}
