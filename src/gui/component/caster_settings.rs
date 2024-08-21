use iced::Command;

use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;

pub struct CasterSettings {}

#[derive(Debug, Clone)]
pub enum Message {
    SelectDisplay(String), //cambiare tipo nel display corrispondente
    GoToConnect,
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        todo!();
    }
}

impl<'a> Component<'a> for CasterSettings {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<crate::gui::app::Message> {
        match message {
            Message::SelectDisplay(display) => Command::none(),
            Message::GoToConnect => {
                //funzioni di backend
                Command::none()
            }
        }
    }

    fn view(
        &self, /*, props: Self::ViewProps*/
    ) -> crate::gui::theme::widget::Element<'_, crate::gui::app::Message> {
        todo!()
    }
}
