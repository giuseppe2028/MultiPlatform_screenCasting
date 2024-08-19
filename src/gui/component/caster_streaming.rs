use iced::widget::{container, image, row};
use iced::Color;
use iced::Length::{self, Fill};

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::Element;

pub struct CasterStreaming {
    // todo!()
}

#[derive(Debug, Clone)]
pub enum Message {
    // todo!()
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        todo!()
    }
}

impl<'a> Component<'a> for CasterStreaming {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        todo!();
        iced::Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let image = Element::from(
            image(format!("./resources/icons/512x512.png"))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .explain(Color::WHITE);

        let buttons = column_iced![
            MyButton::new("exit")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::BackUndo)
                .build()
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            MyButton::new("record")
                .style(Style::Primary)
                .build()
                .padding(8)
        ]
        .padding(8).spacing(10);
        let buttons1 = row![
            MyButton::new("exit")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::BackUndo)
                .build()
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            MyButton::new("record")
                .style(Style::Primary)
                .build()
                .padding(8)
        ]
        .padding(8)
        .spacing(10);

        let sidebar = column_iced![buttons]
            .spacing(8)
            .align_items(iced::Alignment::Center);

        let streaming = container(
            column_iced![image, buttons1]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        );

        container(
            column_iced![row![sidebar, streaming]]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        )
        .into()
    }
}
