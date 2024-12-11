use crate::gui::app;
use crate::gui::component::{Component};
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::{MyButton, Style};
use crate::gui::theme::text::text;
use crate::gui::theme::widget::Element;
use iced::widget::{container, image, row, Image};
use iced::{keyboard::{Event::KeyPressed}, Event, event, window, Size};
use iced::{Command, Subscription};
use iced::keyboard::Key;
use std::sync::{Arc, RwLock};
use iced::window::{Level, Position};
use iced::window::settings::PlatformSpecific;
use tokio::sync::{mpsc::Receiver, Mutex};
use xcap::image::RgbaImage;
use crate::column_iced;
use crate::gui::app::Message;

pub struct CasterStreaming {
    pub toggler: bool,
    pub receiver: Arc<Mutex<Receiver<RgbaImage>>>,
    pub frame_to_update: Arc<Mutex<Option<RgbaImage>>>,
    pub warning_message: bool,
    pub viewrs: Arc<RwLock<usize>>,
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

        // Define the annotation buttons
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

        // Define the control buttons (e.g., play/pause, tools)
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
                .on_press(app::Message::StopStreaming),
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
            .spacing(10);

        // Define the sidebar and streaming layout
        let sidebar = column_iced![annotation_buttons]
            .spacing(8)
            .align_items(iced::Alignment::Center);

        let streaming = container(
            column_iced![image, menu]
                .spacing(8)
                .align_items(iced::Alignment::Center),
        );

        let message = row![text("Your screen is blanking")];
        if self.toggler {
            container(
                column_iced![row![sidebar, streaming]]
                    .spacing(8)
                    .align_items(iced::Alignment::Center),
            )
                .into()
        } else if self.warning_message {
            container(
                column_iced![message, row![streaming]]
                    .spacing(8)
                    .align_items(iced::Alignment::Center),
            )
                .into()
        } else if self.toggler && self.warning_message {
            container(
                column_iced![row![message, sidebar, streaming]]
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
            (Event::Keyboard(KeyPressed { key, .. }), ..) => {
                Some(MessageUpdate::KeyPressed(key))
            }
            _ => None,
        })
    }
}
