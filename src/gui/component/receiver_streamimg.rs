use iced::widget::{container, image, row};
use iced::{Color, Command};
use iced::Length::{self, Fill};

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::Element;

pub struct ReceiverStreaming {
    pub recording: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartRecording(bool),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::StartRecording(message)
    }
}

impl<'a> Component<'a> for ReceiverStreaming {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::StartRecording(new_status) => {
                self.recording = new_status;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let image = Element::from(
            image(format!("./resources/icons/512x512.png"))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .explain(Color::WHITE);

        let buttons = if self.recording {
            row![
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(Icon::StopRecord)
                    .build(30)
                    .on_press(Message::StartRecording(false).into()),
                CircleButton::new("")
                    .style(Style::Danger)
                    .icon(Icon::Cancel)
                    .build(21)
                    .on_press(app::Message::Back(app::Page::ReceiverStreaming)),
            ]
            .spacing(5)
            .padding(8)
        } else {
            row![
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(Icon::StartRecord)
                    .build(24)
                    .on_press(Message::StartRecording(true).into()),
                CircleButton::new("")
                    .style(Style::Danger)
                    .icon(Icon::Cancel)
                    .build(21)
                    .on_press(app::Message::Back(app::Page::ReceiverStreaming)),
            ]
            .spacing(5)
            .padding(8)
        };
        //let screen = column_iced![row![image].spacing(20)];
        container(
            column_iced![image, buttons]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        )
        .into()
    }
}
