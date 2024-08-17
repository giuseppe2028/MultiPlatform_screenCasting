use std::fmt::Debug;

use iced::Command;

use crate::gui::app;
use crate::gui::theme::widget::Element;

pub mod home;
pub mod connection;
pub mod receiver_ip;

pub trait Component<'a> {
    type Message: Into<app::Message> + Clone + Debug;
    //type UpdateProps;
    //type ViewProps;
    
    fn update(&mut self, message: Self::Message) -> Command<app::Message>;
    fn view(&self /*, props: Self::ViewProps*/) -> Element<'_, app::Message>;
}
