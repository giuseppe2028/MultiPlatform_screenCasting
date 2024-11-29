use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row};
use iced::{Command, Subscription};
use iced::Length::Fill;

use crate::column_iced;
use crate::gui::app;
use crate::gui::app::Page;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::Element;
// use crate::gui::theme::widget::TextInput;

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Role {
    Caster,
    Receiver,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChosenRole(Role),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::RoleChosen(message)
    }
}

impl<'a> Component<'a> for Home {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::ChosenRole(role) => match role {
                Role::Caster => {
                    app::Message::RoleChosen(Message::ChosenRole(Role::Caster));
                    Command::none()
                }
                Role::Receiver => {
                    app::Message::RoleChosen(Message::ChosenRole(Role::Receiver));
                    Command::none()
                }

            },
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        container(
            column_iced![
                row![bold("MAKE YOUR CHOICE").size(60)],
                row![
                    MyButton::new("CASTER")
                        .style(Style::Primary)
                        .icon(Icon::CasterHome)
                        .build()
                        .on_press(Self::Message::ChosenRole(Role::Caster).into()),
                    MyButton::new("RECEIVER")
                        .icon(Icon::ReceiverHome)
                        .style(Style::Primary)
                        .build()
                        .on_press(Self::Message::ChosenRole(Role::Receiver).into())
                ],
                row![MyButton::new("Shortcut")
                    .icon(Icon::Tools)
                    .style(Style::Secondary)
                    .build()
                    .on_press(app::Message::Route(Page::Shortcut))]
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

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
