use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row};
use iced::{Command, Subscription};
use iced::Length::Fill;
use crate::column_iced;
use crate::gui::app;
use crate::gui::component::receiver_ip;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::textinput::textinput;
use crate::gui::theme::widget::Element;

pub struct ReceiverIp {
    pub indirizzo_ip: String,
    pub message: String
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeInput(String),
    Pressed(String),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        match message {
            Message::ChangeInput(input) => {
                app::Message::ReceiverInputIp(Message::ChangeInput(input))
            }
            Message::Pressed(ip) => app::Message::ReceiverSharing(ip),
        }
    }
}

impl<'a> Component<'a> for ReceiverIp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::ChangeInput(new_value) => {
                self.indirizzo_ip = new_value;
                self.message = "".to_string();
                Command::none()
            }
            Message::Pressed(_ip) => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        let back_button = container(row![CircleButton::new("")
            .style(Style::Danger)
            .icon(Icon::BackLeft)
            .build(20)
            .on_press(app::Message::Back(app::Page::ReceiverIp))])
            .padding([6, 0, 0, 6])
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top);

        let message = row![crate::gui::theme::text::text(self.message.clone())];

        let main_content = container(
            column_iced![
                row![bold("Insert IP address").size(60)],
                row![textinput("192.168.1.1", self.indirizzo_ip.as_str())
                    .width(300)
                    .size(27)
                    .on_input(|written_ip| {
                        receiver_ip::Message::ChangeInput(written_ip).into()
                    })],
                    message,
                row![MyButton::new("Connect")
                    .style(Style::Primary)
                    .build()
                    .on_press(Message::Pressed(self.indirizzo_ip.clone()).into())]
                .spacing(20)
            ]
                .align_items(iced::Alignment::Center)
                .spacing(20),
        )
            .width(Fill)
            .height(Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        // Unire il pulsante "Back" con il contenuto principale in un layout a strati
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
