use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use iced::widget::{container, image, Image, row, text};
use iced::{Color, ContentFit, Length, Subscription};
use iced::futures::FutureExt;
use scap::frame::Frame;
use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::widget::{Container, Element};
use iced::time::{self, Duration, Instant};
use rand::Rng;

pub struct CasterStreaming {
    pub toggler: bool,
    pub receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
    pub frame_to_update: Arc<Mutex<Option<Vec<u8>>>>,
    pub seconds:u32
}

#[derive(Debug, Clone)]
pub enum MessageUpdate {
    TogglerChanged(bool),
    NewFrame(Vec<u8>),
    Update
}




impl From<MessageUpdate> for app::Message {
    fn from(message: MessageUpdate) -> Self {
        app::Message::TogglerChanged(message)
    }
}

impl<'a> Component<'a> for CasterStreaming {
    type Message = MessageUpdate;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            MessageUpdate::TogglerChanged(new_status) => {
                self.toggler = new_status;
            }
            MessageUpdate::NewFrame(frame) => {
                let mut new_frame = self.frame_to_update.lock().unwrap();
                *new_frame = Some(frame);
            }
            MessageUpdate::Update =>{
                self.seconds += 1;

            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {

        // Ottieni il frame e crea l'immagine
        let image = {
            let frame = self.frame_to_update.lock().unwrap();
            match *frame {
                None => {
                    image(format!("./resources/icons/512x512.png"))
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                }
                Some(ref frame_data) => {
                    // Assicurati che il frame sia in un formato valido
                    Image::new(image::Handle::from_pixels(1440, 900,frame_data.clone())).width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                }
            }

        };

            let stream = Element::from(image).explain(Color::WHITE);
           // let seconds =  text(self.seconds);
            //let seconds =  Element::from(seconds).explain(Color::WHITE);
            let annotation_buttons = column_iced![
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Pencil)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Rubber)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Triangle)
                .build(50)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Square)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Arrow)
                .build(35)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Text)
                .build(25)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
        ]
                .padding(8)
                .spacing(10);
            let menu = row![
            CircleButton::new("tools")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Tools)
                .build(30)
                .padding(8)
                .on_press(app::Message::TogglerChanged(MessageUpdate::TogglerChanged(
                    !self.toggler
                ))),
            CircleButton::new("play/pause")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Pause)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("blank")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Blanking)
                .build(35)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming)),
            CircleButton::new("exit")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::Phone)
                .build(30)
                .padding(8)
                .on_press(app::Message::Close),
        ]
                .align_items(iced::Alignment::Center)
                .padding(8)
                .spacing(10);

            let sidebar = column_iced![annotation_buttons]
                .spacing(8)
                .align_items(iced::Alignment::Center);

            let streaming = container(
                column_iced![stream, menu]
                    .spacing(8)
                    .align_items(iced::Alignment::Center),
            );

            if self.toggler {
                container(
                    column_iced![row![sidebar, streaming]]
                        .spacing(8)
                        .align_items(iced::Alignment::Center),
                )
                    .into()
            } else {
                container(
                    column_iced![row![streaming]]
                        .spacing(8)
                        .align_items(iced::Alignment::Center),
                )
                    .into()
            }
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



