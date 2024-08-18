use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row};
use iced::Command;
use iced::Length::Fill;

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::receiver_ip;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::text::bold;
use crate::gui::theme::textinput::textinput;
use crate::gui::theme::widget::Element;

pub struct ReceiverIp {
    pub(crate) indirizzo_ip: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeInput(String),
    Pressed,
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::ReceiverSharing(message)
    }
}

impl<'a> Component<'a> for ReceiverIp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::ChangeInput(new_value) => {
                self.indirizzo_ip = new_value;
                Command::none()
            }
            Message::Pressed => {
                //connect to IndirizzoIp
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        //inserire esternamente al container il bottone per tornare indietro
        //position fixed in alto a sinistra perOgni finestra
        container(
            column_iced![
                row![bold("Insert IP address").size(60)],
                row![textinput("192.168.1.1", self.indirizzo_ip.as_str())
                    .on_input(|value| receiver_ip::Message::ChangeInput(value).into())],
                row![MyButton::new("Connect")
                    .style(Style::Primary)
                    .build()
                    .on_press(Message::Pressed.into())]
                .spacing(20)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(20),
        )
        .width(Fill)
        .height(Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }
}
