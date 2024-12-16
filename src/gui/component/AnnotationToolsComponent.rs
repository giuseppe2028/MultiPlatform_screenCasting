use futures::StreamExt;
use iced::{Color, Command, Length, Point, Subscription};
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::{Button, Canvas, ColorPicker, Column, Element};
use iced::widget::{button, canvas, Container as CT, container as ct, container, row, Text};
use iced::widget::container::Appearance;
use iced_aw::color_picker;

use crate::column_iced;
use crate::gui::component::Annotation::Square::{ArrowCanva, CanvasWidget, CircleCanva, LineCanva, RectangleCanva, Shape, Status};
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::container::Style;
use crate::gui::theme::button::Style as BT;
use crate::gui::theme::PaletteColor;
use crate::gui::theme::text::text;
use crate::gui::theme::textinput::textinput;

pub struct AnnotationTools {
    pub canvas_widget: CanvasWidget,
    pub setSelectedAnnotation: bool
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

        let textInputForm = container(column_iced![
                row![text("Insert the text you want to display")].padding([0,0,20,0]),
                row![textinput("Type something here...", self.canvas_widget.textSelected.text.as_str())
                .on_input(app::Message::TextCanvasChanged)],
                row![button("save").on_press(app::Message::TextPressed(false))]
        ]).style(
            Style::Container
        ).padding([20, 20,20,20]).width(Length::from(300)).height(Length::Fill);
        // Definizione del vettore annotation_buttons
        let mut annotation_buttons = column_iced![];

        // Condizione per verificare se textPressed è attivo
        if let Status::TextPositioned = self.canvas_widget.text_status {
            let text_input_form = column_iced![ textInputForm];
            annotation_buttons = text_input_form;
        } else {
            annotation_buttons = column_iced![
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
            .on_press(app::Message::SelectShape(Shape::Circle(CircleCanva {
                center: Point::default(),
                radius: 0.0,
            }))),
        CircleButton::new("")
            .style(BT::Primary)
            .icon(crate::gui::theme::icon::Icon::Square)
            .build(30)
            .padding(8)
            .on_press(app::Message::SelectShape(Shape::Rectangle(RectangleCanva {
                startPoint: Default::default(),
                width: 0.0,
                height: 0.0,
            }))),
        CircleButton::new("")
            .style(BT::Primary)
            .icon(crate::gui::theme::icon::Icon::Arrow)
            .build(30)
            .padding(8)
            .on_press(app::Message::SelectShape(Shape::Arrow(ArrowCanva{
                    starting_point:Default::default(),
                    ending_point: Default::default()
                }))),
                CircleButton::new("")
            .style(BT::Primary)
            .icon(crate::gui::theme::icon::Icon::Triangle)
            .build(30)
            .padding(8)
            .on_press(app::Message::SelectShape(Shape::Line(LineCanva{
                    starting_point:Default::default(),
                    ending_point: Default::default()
                }))),
        CircleButton::new("")
            .style(BT::Primary)
            .icon(crate::gui::theme::icon::Icon::Text)
            .build(25)
            .padding(8)
            .on_press(app::Message::TextPressed(true)),
    ]
                .padding(8)
                .spacing(10);
        }


        // Define the sidebar and streaming layout
        let mut sidebar = column_iced![
            row![
                annotation_buttons,Canvas::new(&self.canvas_widget).width(Length::Fill).height(Length::Fill)
            ]
        ]
            .spacing(8)
            .align_items(iced::Alignment::Center);
        if self.setSelectedAnnotation {
            sidebar = sidebar.push(row![text("Press where do you want to put the note")])
        }

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

