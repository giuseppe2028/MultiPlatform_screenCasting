use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::{Button, Canvas, Container, Element, Text};
use iced::{event, Color, Command, Event, Length, Point, Subscription};
use iced::widget::container;
use iced::advanced::graphics::core::window;
use iced_aw::color_picker;
use crate::column_iced;

pub struct ColorPickerWindow {
    pub(crate) selected_color:Color,
    pub window_id:Option<window::Id> 
}

#[derive(Debug, Clone)]
pub enum MessageColorPicker {
    CloseRequested,
}

impl From<MessageColorPicker> for app::Message {
    fn from(message: MessageColorPicker) -> Self {
        match message {
            _ => app::Message::CloseRequestedColorPicker,
        }
    }
}

impl<'a> Component<'a> for ColorPickerWindow {
    type Message = MessageColorPicker;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            MessageColorPicker::CloseRequested => {
                app::Message::CloseRequestedColorPicker;
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
        if let Some(_) = self.window_id {
             event::listen_with(|event, _status| match event {
                 Event::Window(_id, window::Event::Closed) => {
                    println!("t;rigger");
                    Some(MessageColorPicker::CloseRequested)
                 },
                 _ => None,
             })
         } else {
             Subscription::none()
         }
     }
}
