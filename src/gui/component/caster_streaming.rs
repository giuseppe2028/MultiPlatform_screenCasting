use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use enigo::Key;
use iced::widget::{container, image, Image, row};
use iced::{Color, Subscription};
use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::widget::Element;
use iced::{
    event::{self},
    keyboard::{KeyCode, Event::KeyPressed},
    Event
};
use xcap::image::RgbaImage;
use crate::gui::component::shorcut::Shortcut;
use crate::model::Shortcut::{from_str_to_key_code, ShortcutController};

pub struct CasterStreaming {
    pub toggler: bool,
    pub receiver: Arc<Mutex<Receiver<RgbaImage>>>,
    pub frame_to_update: Arc<Mutex<Option<RgbaImage>>>,
    pub measures: (u32, u32),//width, height
    pub shortcut: ShortcutController
}

#[derive(Debug, Clone)]
pub enum MessageUpdate {
    TogglerChanged(bool),
    NewFrame(RgbaImage),
    KeyPressed(KeyCode),
    StopStreaming
}




impl From<MessageUpdate> for app::Message {

    fn from(message: MessageUpdate) -> Self {
        match message {
            MessageUpdate::TogglerChanged(_) => {app::Message::TogglerChanged(message)}
            MessageUpdate::NewFrame(_) => {
                app::Message::None
            }
            MessageUpdate::StopStreaming => {
                app::Message::StopStreaming
            }
            _ => {app::Message::StopStreaming}
        }

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
            MessageUpdate::StopStreaming=> {

            }
            MessageUpdate::KeyPressed(key_code) => {
                if key_code == self.shortcut.get_blanking_screen_shortcut() {
                    //manda messaggio
                }
                else if key_code == self.shortcut.get_manage_trasmition_shortcut() {
                    app::Message::StopStreaming;
                }
                else if key_code == self.shortcut.get_terminate_session_shortcut() {
                    //manda relativo messaggio
                }
                
            },
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
                    Image::new(image::Handle::from_pixels(frame_data.width(), frame_data.height(),frame_data.clone().into_raw())).width(iced::Length::Fill)
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
        iced::subscription::events_with(|event, status| match (event, status) {
            // Scorciatoia: Space -> StopStreaming
            (
                Event::Keyboard(KeyPressed { key_code , .. }), event::Status::Ignored
            ) => {
                Some(MessageUpdate::KeyPressed(key_code))
            },
            _ => None,
        })
    }
}




