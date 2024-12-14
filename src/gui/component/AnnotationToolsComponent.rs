use futures::StreamExt;
use iced::{Color, Command, Length, Point, Subscription};
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::{Button, Canvas, ColorPicker, Column, Element};
use iced::widget::{canvas, Container as CT, container as ct, Text};
use iced_aw::color_picker;
use crate::column_iced;
use crate::gui::component::Annotation::Square::{CanvasWidget, CircleCanva, RectangleCanva, Shape};
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::container::Style;
use crate::gui::theme::button::Style as BT;



pub struct AnnotationTools {
    pub canvas_widget: CanvasWidget
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
        let but = Button::new(Text::new("Set Color")).on_press(app::Message::ChooseColor);
        let color_submit_callback = |color: Color| app::Message::SubmitColor(color);


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
                .on_press(app::Message::ClearShape),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Triangle)
                .build(30)
                .padding(8)
                .on_press(app::Message::SelectShape(Shape::Circle(CircleCanva{
                center:Point::default(),
                radius:0.0
            }))),
            CircleButton::new("")
                .style(BT::Primary)
                .icon(crate::gui::theme::icon::Icon::Square)
                .build(30)
                .padding(8)
                .on_press(app::Message::SelectShape(Shape::Rectangle(RectangleCanva{
                
            startPoint: Default::default(),width: 0.0 , height: 0.0 , }))),
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
           // ColorPicker::new(true, Default::default(), but.into(),app::Message::CancelColor, |color: Color| app::Message::SubmitColor(color)),
        ]
            .padding(8)
            .spacing(10);
        // Define the sidebar and streaming layout
        let sidebar = column_iced![annotation_buttons,Canvas::new(&self.canvas_widget).width(Length::Fill).height(Length::Fill)]
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

