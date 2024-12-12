use futures::StreamExt;
use iced::{Command, Length, Subscription};
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::{Canvas, Column, Element};
use iced::widget::{canvas, Container as CT, container as ct, Text};
use crate::column_iced;
use crate::gui::component::Annotation::Square::{ SquareCanva};
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::container::Style;
use crate::gui::theme::button::Style as BT;



pub struct AnnotationTools {

}

#[derive(Debug, Clone)]
pub enum Message {

}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        match message {
            _ => {app::Message::None}
        }
    }
}

impl<'a> Component<'a> for AnnotationTools {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {

        // Define the annotation buttons
        let annotation_buttons = column_iced![
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Pencil)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Rubber)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Triangle)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Square)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Arrow)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Text)
                .build(25)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
        ]
            .padding(8)
            .spacing(10);
        // Define the sidebar and streaming layout
        let sidebar = column_iced![annotation_buttons,Canvas::new(SquareCanva::new(30.))]
            .spacing(8)
            .align_items(iced::Alignment::Center);


      CT::new(sidebar)
            .style(Style::Window)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()


    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}

