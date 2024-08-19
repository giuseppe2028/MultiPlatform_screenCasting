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

pub struct ReceiverStreaming {
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

impl<'a> Component<'a> for ReceiverStreaming {
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

        let buttons = row![
            MyButton::new("exit")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::BackUndo)
                .build().on_press(app::Message::Back(app::Page::ReceiverStreaming)),
            MyButton::new("record").style(Style::Primary).build()
        ].padding(8);
        //let screen = column_iced![row![image].spacing(20)];
        container(column_iced![image, buttons].spacing(8).align_items(iced::Alignment::Center)).into()
    }
}
