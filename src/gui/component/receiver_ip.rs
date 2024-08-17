use iced::widget::container;

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::widget::Element;

pub struct Receiver_ip {

}

#[derive(Debug, Clone)]
pub enum Message {
    //todo
    Todo
}


 impl From<Message> for app::Message {
     fn from(message: Message) -> Self {
        todo!()
    }
 }

impl<'a> Component<'a> for Receiver_ip {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::Todo => todo!(),
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        //todo
        todo!()
    }
}
