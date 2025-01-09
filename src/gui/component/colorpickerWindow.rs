use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::{Button, Canvas, Container, Element, Text};
use iced::{event, Color, Command, Event, Length, Point, Subscription};
use iced::widget::container;
use iced_aw::color_picker;
use crate::column_iced;

pub struct ColorPickerWindow {
    pub(crate) selected_color:Color
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

impl<'a> Component<'a> for ColorPickerWindow {
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
        let but = Button::new(Text::new("Set Color")).on_press(app::Message::SetColor);
        let color_picker = color_picker(
            true,
            self.selected_color,
            but,
            app::Message::CancelColor,
            app::Message::SubmitColor,
        );
        container(color_picker)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
