use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row};
use iced::Length::Fill;
use iced::Subscription;
use crate::column_iced;
use crate::gui::component::Component;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::{bold, text};
use crate::gui::theme::button::circle_button::CircleButton;

use crate::gui::app;
use crate::gui::theme::button::MyButton;

pub struct Connection {
    pub ip_address: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartSharing,
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

    fn view(&self) -> crate::gui::theme::widget::Element<'_, app::Message> {
        let back_button = container(row![CircleButton::new("")
            .style(Style::Danger)
            .icon(Icon::BackLeft)
            .build(20)
            .on_press(app::Message::Back(app::Page::Connection))])
        .padding([6, 0, 0, 6])
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top);

        let main_content = container(
            column_iced![
                row![bold("Your IP address").size(50)].align_items(iced::Alignment::Center),
                row![text(self.ip_address.clone()).size(30)].align_items(iced::Alignment::Center),
                row![MyButton::new("CONNECT")
                    .style(Style::Primary)
                    .build()
                    .on_press(Message::StartSharing.into())]
                .align_items(iced::Alignment::Center)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(20),
        )
        .width(Fill)
        .height(Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

        container(column_iced![
            back_button,  // Il pulsante back è al primo posto e separato dal resto
            main_content  // Contenuto principale
        ])
        .width(Fill)
        .height(Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
