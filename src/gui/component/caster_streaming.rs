use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::text::text;
use crate::gui::theme::widget::{Column, Element, Row};
use crate::model::Shortcut::ShortcutController;
use iced::widget::{container, image, row, Image,column};
use iced::{keyboard::{Event::KeyPressed}, Event, event};
use iced::{Command, Subscription};
use std::sync::Arc;
use iced::keyboard::Key;
use tokio::sync::{mpsc::Receiver, Mutex};
use xcap::image::RgbaImage;

pub struct CasterStreaming {
    pub toggler: bool,
    pub receiver: Arc<Mutex<Receiver<RgbaImage>>>,
    pub frame_to_update: Arc<Mutex<Option<RgbaImage>>>,
    pub measures: (u32, u32), // width, height
    pub is_loading: bool,
    pub shortcut: ShortcutController,
    pub warning_message: bool,
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
                println!("{:?}", code);
                app::Message::KeyShortcut(code)
            }

            _ => {
                println!("ENTRO");
                println!("{:?}", message);
                app::Message::Close
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
                iced::Command::none()
            }
            MessageUpdate::NewFrame(frame) => {
                //println!("{:?}", frame);
                self.is_loading = false;
                *self.frame_to_update.blocking_lock() = Some(frame);
                self.is_loading = false;
                Command::none()
            }
            MessageUpdate::KeyPressed(key_code) => {
                /*if key_code == self.shortcut.get_blanking_screen_shortcut() {
                    //manda messaggio
                    print!("ciaoooo");
                }
                else if key_code == self.shortcut.get_manage_trasmition_shortcut() {
                    print!("ciaooooSoooocaaaa");

                }
                else if key_code == self.shortcut.get_terminate_session_shortcut() {
                    print!("123 ale 123");
                   app::Message::Close;
                }*/
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
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
        let annotation_buttons =Column::new().push(
            CircleButton::new("")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Pencil)
                .build(30)
                .padding(8)
                .on_press(app::Message::Back(app::Page::CasterStreaming))
        )
            .push(
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Rubber)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            )
            .push(
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Triangle)
                    .build(50)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            )
            .push(
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Square)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            )
            .push(
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Arrow)
                    .build(35)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            )
            .push(
                CircleButton::new("")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Text)
                    .build(25)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            );


        // Define the control buttons (e.g., play/pause, tools)
        let menu = Row::new().push(
            CircleButton::new("tools")
                .style(Style::Primary)
                .icon(crate::gui::theme::icon::Icon::Tools)
                .build(30)
                .padding(8)
                .on_press(app::Message::TogglerChanged(MessageUpdate::TogglerChanged(
                    !self.toggler
                )))
        )
            .push(
                CircleButton::new("play/pause")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Pause)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Back(app::Page::CasterStreaming))
            )
            .push(
                CircleButton::new("blank")
                    .style(Style::Primary)
                    .icon(crate::gui::theme::icon::Icon::Blanking)
                    .build(35)
                    .padding(8)
                    .on_press(app::Message::Blanking)
            )
            .push(
                CircleButton::new("exit")
                    .style(Style::Danger)
                    .icon(crate::gui::theme::icon::Icon::Phone)
                    .build(30)
                    .padding(8)
                    .on_press(app::Message::Close)
            )
        .align_items(iced::Alignment::Center)
        .padding(8)
        .spacing(10);
        // Define the sidebar and streaming layout
        let sidebar = Column::new().push(annotation_buttons);

        let streaming = container(
            Column::new().push(
                Row::new().push(image).push(menu)
            )
        );

        let message = row![text("Your screen is blanking")];
        if self.toggler {
            container(
                Column::new().push(
                    Row::new().push(sidebar).push(streaming)
                )
            )
            .into()
        } else if self.warning_message {
            container(
                Column::new().push(
                    Row::new().push(message).push(Row::new().push(streaming))
                )
            )
            .into()
        } else if self.toggler && self.warning_message {
            container(
                Column::new().push(
                    Row::new().push(Row::new().push(message).push(sidebar).push(streaming))
                )
            )
            .into()
        } else {
            container(
                Column::new().push(
                    Row::new().push(Row::new().push(streaming))
                )
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
