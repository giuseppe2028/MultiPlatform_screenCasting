use crate::column_iced;
use crate::gui::app::{self, Modality};
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::{MyButton, Style};
use crate::gui::theme::text::text;
use crate::gui::theme::widget::Element;
use iced::keyboard::Key;
use iced::widget::{container, image, row, Image};
use iced::{event, keyboard::Event::KeyPressed, Event};
use iced::{Command, Subscription};
use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc::Receiver, Mutex};
use xcap::image::RgbaImage;

pub struct CasterStreaming {
    pub toggler: bool,
    pub receiver: Arc<Mutex<Receiver<RgbaImage>>>,
    pub frame_to_update: Arc<Mutex<Option<RgbaImage>>>,
    pub warning_message: bool,
    pub modality: Modality,
    pub viewrs: Arc<RwLock<usize>>,
    pub stop: bool,
}

#[derive(Debug, Clone)]
pub enum MessageUpdate {
    TogglerChanged(bool),
    NewFrame(RgbaImage),
    KeyPressed(Key),
}

impl From<MessageUpdate> for app::Message {
    fn from(message: MessageUpdate) -> Self {
        match message {
            MessageUpdate::TogglerChanged(_) => app::Message::TogglerChanged(message),
            MessageUpdate::NewFrame(_) => app::Message::None,
            MessageUpdate::KeyPressed(code) => {
                println!("sono in key pressed {:?}", code);
                app::Message::KeyShortcut(code)
            }
        }
    }
}

impl<'a> Component<'a> for CasterStreaming {
    type Message = MessageUpdate;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            MessageUpdate::TogglerChanged(new_status) => {
                self.toggler = new_status;
                Command::none()
            }
            MessageUpdate::NewFrame(frame) => {
                //println!("{:?}", frame);
                *self.frame_to_update.blocking_lock() = Some(frame);
                Command::none()
            }
            MessageUpdate::KeyPressed(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        let viewrs = self.viewrs.read().unwrap();
        // Get the current frame and create an image
        let image = {
            let frame = self.frame_to_update.blocking_lock();
            match *frame {
                None => {
                    println!("Niente da fare");
                    image(format!("./resources/icons/512x512.png"))
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                }
                Some(ref frame_data) => {
                    // Assicurati che il frame sia in un formato valido
                    Image::new(image::Handle::from_pixels(
                        frame_data.width(),
                        frame_data.height(),
                        frame_data.clone().into_raw(),
                    ))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                }
            }
        };

        let pause_button = if self.toggler {
            if self.stop {
                CircleButton::new("play/pause")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Play)
                    .build(30)
                    .padding(8)
                //.on_press(app::Message::StopStreaming)
            } else {
                CircleButton::new("play/pause")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Pause)
                    .build(30)
                    .padding(8)
                //.on_press(app::Message::StopStreaming)
            }
        } else {
            if self.stop {
                CircleButton::new("play/pause")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Play)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::StopStreaming)
            } else {
                CircleButton::new("play/pause")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Pause)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::StopStreaming)
            }
        };

        // Define the control buttons (e.g., play/pause, tools)
        let menu = if !self.toggler {
            row![
                CircleButton::new("tools")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Tools)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::TogglerChanged(MessageUpdate::TogglerChanged(
                        !self.toggler
                    ))),
                pause_button,
                CircleButton::new("blank")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Blanking)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Blanking),
                CircleButton::new("exit")
                    .style(Style::Danger)
                    .icon(crate::gui::theme::icon::Icon::Phone)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Close),
                MyButton::new(&format!("{}", viewrs))
                    .style(Style::Secondary)
                    .icon(crate::gui::theme::icon::Icon::Viewers)
                    .build()
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming)),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8)
            .spacing(10)
        } else {
            row![
                CircleButton::new("tools")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Tools)
                .build(30)
                .padding(8),
                pause_button,
                CircleButton::new("blank")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Blanking)
                    .build(30)
                    .padding(8),
                CircleButton::new("exit")
                    .style(Style::Danger)
                    .icon(crate::gui::theme::icon::Icon::Phone)
                    .build(30)
                    .padding(8),
                MyButton::new(&format!("{}", viewrs))
                    .style(Style::Secondary)
                    .icon(crate::gui::theme::icon::Icon::Viewers)
                    .build()
                    .padding(8),
            ]
            .align_items(iced::Alignment::Center)
            .padding(8)
            .spacing(10)
        };

        let streaming = container(
            column_iced![image, menu]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        );

        let message = row![text("Your screen is blanking")];
        if self.warning_message {
            container(
                column_iced![message, row![streaming]]
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
        event::listen_with(|event, status| match (event, status) {
            // Scorciatoia: Space -> StopStreaming
            (Event::Keyboard(KeyPressed { key, .. }), ..) => Some(MessageUpdate::KeyPressed(key)),
            _ => None,
        })
    }
}
