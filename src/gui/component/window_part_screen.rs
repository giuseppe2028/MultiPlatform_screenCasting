use enigo::{Enigo, Mouse, Settings};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Image, image, mouse_area, row, text};
use iced::{Application, Command, Event, Subscription};
use iced::Length::Fill;
use iced::mouse::Event::CursorMoved;
use iced::widget::mouse_area::MouseArea;
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
    FirstPress(i32, i32),
    SecondPress(i32, i32),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        match message {
            Message::FirstPress(x, y) => {
                app::Message::AreaSelectedFirst(x,y)
            }
            Message::SecondPress(x, y) => {
                app::Message::AreaSelectedSecond(x,y)
            }
        }
    }
}

impl<'a> Component<'a> for WindowPartScreen {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {

        match message {
            Message::FirstPress(x, y) => {
                println!("ciao");
                self.coordinate[0] =(x, y)
            }
            Message::SecondPress(x, y) => {
                self.coordinate[1] =(x, y)
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        mouse_area(
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
            .on_press({
                let points = enigo.location().unwrap();
                 Message::FirstPress(points.0, points.1).into()
            })

            .on_release({
                let points = enigo.location().unwrap();
                 Message::SecondPress(points.0, points.1).into()
            })
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
