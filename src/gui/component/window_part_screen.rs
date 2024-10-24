use enigo::{Enigo, Mouse, Settings};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Image, image, row, text,mouse_area};
use iced::{Command, Event, Subscription};
use iced::Length::Fill;
use iced::mouse::Event::CursorMoved;
use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::{Container, Element};
// use crate::gui::theme::widget::TextInput;

pub struct WindowPartScreen {
    pub screenshot:Vec<u8>,
    pub(crate) coordinate:[(i32, i32);2]
}

#[derive(Debug, Clone)]
pub enum Role {
    Caster,
    Receiver,
}

#[derive(Debug, Clone)]
pub enum Message {
    FirstPress,
    SecondPress,
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::Close
    }
}

impl<'a> Component<'a> for WindowPartScreen {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        match message {
            Message::FirstPress => {
                self.coordinate[0] =enigo.location().unwrap()
            }
            Message::SecondPress => {
                self.coordinate[1] =enigo.location().unwrap()
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {


        let mouse = mouse_area(
            column_iced![
                row![
                    Image::new(image::Handle::from_pixels(1440, 900,rgb_to_rgba(self.screenshot.clone()))).width(iced::Length::Fill)
                .height(iced::Length::Fill),
                text(format!(
                    "ciao x1: {} y1: {} x2: {} y2: {}",
                    self.coordinate[0].0,  // x1
                    self.coordinate[0].1,  // y1
                    self.coordinate[1].0,  // x2
                    self.coordinate[1].1   // y2
                ))
                ]
            ]
        )
            .on_press(Message::FirstPress)
            .on_release(Message::SecondPress);
      let element = Element::from(
          mouse
      );
        container(
            column_iced![element]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        )
            .into()



        /*container(
            column_iced![
            row![text("Seleziona la parte di schermo da visualizzare").size(30)],
            row![


                MyButton::new("Connect")
                    .style(Style::Primary)
                    .build()
                    .on_press(Self::Message::Moment.into()),  // Proper message handling
            ]
            .spacing(20)
        ]
                .align_items(iced::Alignment::Center)
                .spacing(20),
        )
            .width(Fill)
            .height(Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into() // Properly converting container to Element<app::Message>*/
    }


    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}

fn rgb_to_rgba(rgb_buffer: Vec<u8>) -> Vec<u8> {
    let rgb_len = rgb_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((rgb_len / 3) * 4); // Ogni pixel RGB diventa RGBA

    // Itera i pixel RGB e aggiungi il canale Alpha
    for rgb_chunk in rgb_buffer.chunks_exact(3) {
        rgba_buffer.push(rgb_chunk[0]); // Red
        rgba_buffer.push(rgb_chunk[1]); // Green
        rgba_buffer.push(rgb_chunk[2]); // Blue
        rgba_buffer.push(255);          // Alpha (opaco)
    }

    rgba_buffer
}
