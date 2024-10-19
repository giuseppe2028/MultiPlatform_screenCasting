use iced::widget::{container, image, row};
use iced::Color;

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::widget::Element;

pub struct CasterStreaming {
    pub toggler: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglerChanged(bool),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::TogglerChanged(message)
    }
}

impl<'a> Component<'a> for CasterStreaming {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::TogglerChanged(new_status) => {
                self.toggler = new_status;
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let image = Element::from(
            image(format!("./resources/icons/512x512.png"))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .explain(Color::WHITE);

        let annotation_buttons = column_iced![
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Pencil)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Rubber)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Triangle)
                .build(50)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Square)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Arrow)
                .build(35)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Text)
                .build(25)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
        ]
        .padding(8)
        .spacing(10);

        let menu = row![
            CircleButton::new("tools")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Tools)
                .build(30)
                .padding(8)
                .on_press(app::Message::TogglerChanged(Message::TogglerChanged(
                    !self.toggler
                ))),
            CircleButton::new("play/pause")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Pause)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("blank")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Blanking)
                .build(35)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("exit")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::Phone)
                .build(30)
                .padding(8)
                .on_press(app::Message::Close),
        ]
        .align_items(iced::Alignment::Center)
        .padding(8)
        .spacing(10);

        let sidebar = column_iced![annotation_buttons]
            .spacing(8)
            .align_items(iced::Alignment::Center);

        let streaming = container(
            column_iced![image, menu]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        );

        if self.toggler {
            container(
                column_iced![row![sidebar, streaming]]
                    .spacing(8)
                    .align_items(iced::Alignment::Center),
            )
            .into()
        } else {
            container(
                column_iced![row![streaming]]
                    .spacing(8)
                    .align_items(iced::Alignment::Center),
            )
            .into()
        }
    }
}
