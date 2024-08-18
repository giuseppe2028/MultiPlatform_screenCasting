use iced::widget::{container, row};
use iced::Length::Fill;
use iced::alignment::{Horizontal, Vertical};

use crate::column_iced;
use crate::gui::component::Component;
use crate::gui::theme::text::{bold, text};
use crate::gui::theme::textinput::textinput;
use crate::gui::theme::button::Style;

use crate::gui::theme::button::MyButton;
use crate::gui::app;

pub struct Connection {
    pub ip_address: String
}

#[derive(Debug, Clone)]
pub enum Message {
    StartSharing
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::StartSharing
    }
}

impl<'a> Component<'a> for Connection {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<crate::gui::app::Message> {
        match message {
            Message::StartSharing => todo!(),
        }
    }
    
    fn view(&self /*, props: Self::ViewProps*/) -> crate::gui::theme::widget::Element<'_, crate::gui::app::Message> {
        container(
            column_iced![
                row![bold("Your IP address").size(50)].align_items(iced::Alignment::Center),
                row![text(self.ip_address.clone()).size(30)].align_items(iced::Alignment::Center),
                row![MyButton::new("CONNECT")
                    .style(Style::Primary)
                    .build()
                    .on_press(Message::StartSharing.into())].align_items(iced::Alignment::Center)
            ].align_items(iced::Alignment::Center).spacing(20)
        )
        .width(Fill)
        .height(Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }
}