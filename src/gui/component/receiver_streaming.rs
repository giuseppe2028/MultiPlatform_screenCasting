use iced::widget::{container, image, row, Image};
use iced::{Command, Subscription};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::widget::Element;
use xcap::image::RgbaImage;

pub struct ReceiverStreaming {
    pub recording: bool,
    pub receiver: Arc<Mutex<Receiver<RgbaImage>>>,
    pub frame_to_update: Arc<Mutex<Option<RgbaImage>>>,
    pub is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum UpdateMessage {
    StartRecording(bool),
    NewFrame(RgbaImage),
}

impl From<UpdateMessage> for app::Message {
    fn from(message: UpdateMessage) -> Self {
        app::Message::StartRecording(message)
    }
}

impl<'a> Component<'a> for ReceiverStreaming {
    type Message = UpdateMessage;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            UpdateMessage::StartRecording(new_status) => {
                Command::none()
            }
            UpdateMessage::NewFrame(frame) => {
                self.is_loading = false;
                *self.frame_to_update.blocking_lock() = Some(frame);
                self.is_loading = false;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        // Ottieni il frame e crea l'immagine
        let image = {
            let frame = self.frame_to_update.blocking_lock();
            match *frame {
                None => {
                    image(format!("./resources/icons/512x512.png"))
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                }
                Some(ref frame_data) => {
                    // Assicurati che il frame sia in un formato valido
                    Image::new(image::Handle::from_pixels(frame_data.width(), frame_data.height(),frame_data.clone().into_raw())).width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                }

            }

        };

        //let stream = Element::from(image).explain(Color::WHITE);

        let buttons = if self.recording {
            row![
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(Icon::StopRecord)
                    .build(30)
                    .on_press(UpdateMessage::StartRecording(false).into()),
                CircleButton::new("")
                    .style(Style::Danger)
                    .icon(Icon::Cancel)
                    .build(21)
                    .on_press(app::Message::Back(app::Page::ReceiverStreaming)),
            ]
            .spacing(5)
            .padding(8)
        } else {
            row![
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(Icon::StartRecord)
                    .build(24)
                    .on_press(UpdateMessage::StartRecording(true).into()),
                CircleButton::new("")
                    .style(Style::Danger)
                    .icon(Icon::Cancel)
                    .build(21)
                    .on_press(app::Message::Close),
            ]
            .spacing(5)
            .padding(8)
        };
        //let screen = column_iced![row![image].spacing(20)];
        container(
            column_iced![image, buttons]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
