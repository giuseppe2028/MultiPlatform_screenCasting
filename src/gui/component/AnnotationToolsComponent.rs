use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Annotation::Square::{
    ArrowCanva, CanvasWidget, CircleCanva, LineCanva, RectangleCanva, Shape, Status,
};
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::{MyButton, Style as BT};
use crate::gui::theme::container::Style;
use crate::gui::theme::text::text;
use crate::gui::theme::textinput::textinput;
use crate::gui::theme::widget::{Canvas, Element};
use iced::advanced::graphics::core::window;
use iced::widget::{container, row, Container as CT};
use iced::{event, Color, Command, Event, Length, Point, Subscription};

pub struct AnnotationTools {
    pub canvas_widget: CanvasWidget,
    pub set_selected_annotation: bool,
    pub selected_color: Color,
    pub show_color_picker: bool,
    pub window_id: Option<window::Id>,
}

#[derive(Debug, Clone)]
pub enum MessageAnnotation {
    CloseRequested,
}

impl From<MessageAnnotation> for app::Message {
    fn from(message: MessageAnnotation) -> Self {
        match message {
            _ => app::Message::CloseRequested,
        }
    }
}

impl<'a> Component<'a> for AnnotationTools {
    type Message = MessageAnnotation;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            MessageAnnotation::CloseRequested => {
                app::Message::CloseRequested;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<'_, app::Message> {

        let text_input_form = container(column_iced![
            row![text("Insert the text you want to display")].padding([0, 0, 20, 0]),
            row![textinput(
                "Type something here...",
                self.canvas_widget.textSelected.text.as_str()
            )
            .on_input(app::Message::TextCanvasChanged)],
            row![].height(10),
            row![MyButton::new("SAVE")
                .style(BT::Primary)
                .build()
                .on_press(app::Message::TextPressed(false))]
        ])
        .style(Style::Container)
        .padding([20, 20, 20, 20])
        .width(Length::from(300))
        .height(Length::Fill);
        // Definizione del vettore annotation_buttons
        let mut annotation_buttons = container(column_iced![]);

        // Condizione per verificare se textPressed è attivo
        if let Status::TextPositioned = self.canvas_widget.text_status {
            let text_input_form = text_input_form;
            annotation_buttons = text_input_form;
        } else {
            annotation_buttons = container(
                column_iced![
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Rubber)
                        .build(30)
                        .padding(8)
                        .on_press(app::Message::ClearShape),
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Circle)
                        .build(30)
                        .padding(8)
                        .on_press(app::Message::SelectShape(Shape::Circle(CircleCanva {
                            center: Point::new(0., 0.),
                            radius: 0.0,
                        }))),
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Square)
                        .build(30)
                        .padding(8)
                        .on_press(app::Message::SelectShape(Shape::Rectangle(
                            RectangleCanva {
                                startPoint: std::default::Default::default(),
                                width: 0.0,
                                height: 0.0,
                            }
                        ))),
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Arrow)
                        .build(30)
                        .padding(8)
                        .on_press(app::Message::SelectShape(Shape::Arrow(ArrowCanva {
                            starting_point: std::default::Default::default(),
                            ending_point: std::default::Default::default()
                        }))),
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Line)
                        .build(30)
                        .padding(8)
                        .on_press(app::Message::SelectShape(Shape::Line(LineCanva {
                            starting_point: std::default::Default::default(),
                            ending_point: std::default::Default::default()
                        }))),
                    CircleButton::new("")
                        .style(BT::Primary)
                        .icon(crate::gui::theme::icon::Icon::Text)
                        .build(25)
                        .padding(8)
                        .on_press(app::Message::TextPressed(true)),
                ]
                .padding(8)
                .spacing(10),
            )
            .style(Style::Container)
            .height(Length::FillPortion(1))
        }

        // Define the sidebar and streaming layout
        let sidebar = column_iced![row![
            annotation_buttons,
            Canvas::new(&self.canvas_widget)
                .width(Length::Fill)
                .height(Length::Fill)
        ]]
        .spacing(8)
        .align_items(iced::Alignment::Center);


        CT::new(sidebar)
            .style(Style::Window)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if let Some(_) = self.window_id {
            event::listen_with(|event, _status| match event {
                Event::Window(_id, window::Event::Closed) => Some(MessageAnnotation::CloseRequested),
                _ => None,
            })
        } else {
            Subscription::none()
        }
    }
}
