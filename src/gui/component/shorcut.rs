use std::fmt;

use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::{self};
use iced::widget::{container, pick_list, row, Row, Space, text};
use iced::{Alignment, Command};
use iced::Length::Fill;
use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::{Column, Container, Element};

use super::keycodeutils::get_keycode_list;

pub struct Shortcut{
    pub manage_transmission: String,
    pub blancking_screen: String,
    pub terminate_session: String,
    pub err_key_set:bool,
}

#[derive(Debug, Clone)]
pub enum Shortcuts {
    ManageTransmission(String),
    BlanckingScreen(String),
    TerminateSession(String),
}

#[derive(Debug, Clone)]
pub enum ShortcutMessage {
    ManageTransmission(String),
    BlanckingScreen(String),
    TerminateSession(String),
}

impl From<ShortcutMessage> for app::Message {
    fn from(message: ShortcutMessage) -> Self {
        match message {
            ShortcutMessage::ManageTransmission(key) => {
                app::Message::ChosenShortcuts(Shortcuts::ManageTransmission(key))
            }
            ShortcutMessage::BlanckingScreen(key) => {
                app::Message::ChosenShortcuts(Shortcuts::BlanckingScreen(key))
            }
            ShortcutMessage::TerminateSession(key) => {
                app::Message::ChosenShortcuts(Shortcuts::TerminateSession(key))
            }
        }
    }
}

impl<'a> Component<'a> for Shortcut{
    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            ShortcutMessage::ManageTransmission(key) => {
                self.manage_transmission = key;
                Command::none()
            }
            ShortcutMessage::BlanckingScreen(key) => {
                self.blancking_screen = key;
                Command::none()
            }
            ShortcutMessage::TerminateSession(key) => {
                self.terminate_session = key;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, app::Message> {
        let back_button = container(row![CircleButton::new("")
            .style(Style::Danger)
            .icon(Icon::BackLeft)
            .build(20)
            // probabilmente errata la riga succesiva, dovrebbe essere app::Page::Home ma non va...
            // mentre funziona con app::Page::CasterSettings e app::Page::ReceiverStreaming
            .on_press(app::Message::Back(app::Page::Shortcut))])
            .padding([6, 0, 0, 6])
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top);

        let manage_transmission_shortcut = pick_list(
            get_keycode_list()
                .iter()
                .filter(|key| {
                    key != &&self.blancking_screen && key != &&self.terminate_session
                })
                .cloned() // Clona i valori per ottenere un Vec<String>
                .collect::<Vec<String>>(), // Raccoglie in un Vec
            Some(self.manage_transmission.clone()), // Passa direttamente la stringa clonata
            move |key| app::Message::ChosenShortcuts(Shortcuts::ManageTransmission(key)),
        ).width(120.);

        let terminate_session_shortcut = pick_list(
            get_keycode_list()
                .iter()
                .filter(|key| {
                    key != &&self.blancking_screen && key != &&self.manage_transmission
                })
                .cloned() // Clona i valori per ottenere un Vec<String>
                .collect::<Vec<String>>(),
            Some(self.terminate_session.clone()),
            move |key| app::Message::ChosenShortcuts(Shortcuts::TerminateSession(key)),
        ).width(120.);
        let blanking_screen_shortcut = pick_list(
            get_keycode_list()
                .iter()
                .filter(|key| {
                    key != &&self.manage_transmission && key != &&self.terminate_session
                })
                .cloned() // Clona i valori per ottenere un Vec<String>
                .collect::<Vec<String>>(),
            Some(self.blancking_screen.clone()),
            move |key| app::Message::ChosenShortcuts(Shortcuts::BlanckingScreen(key)),
        ).width(120.);


        container(
            column_iced![
                back_button,
                container(column_iced![
                             row![bold("Costumize your Shortcuts").size(40)].padding([0,0,0,70]).align_items(Alignment::Center),
                    row![
                        column_iced![
                            row![text("Choose a key for pausing/resuming the trasmission").vertical_alignment(Vertical::Center),Space::new(0,20),],
                            row![text("Choose a key for blanking the screen").vertical_alignment(Vertical::Center).height(80),],
                            row![text("Choose a key for terminating the current session").vertical_alignment(Vertical::Center),Space::new(0,20)]
                        ],
                        column_iced![
                             row![Space::new(20,0),manage_transmission_shortcut].padding([20,0,20,0]),
                    row![Space::new(20,0),blanking_screen_shortcut].padding([0,0,20,0]),
                    row![Space::new(20,0),terminate_session_shortcut].padding([0,0,20,0]),
                        ]
                    ].align_items(Alignment::Center),
                    //bold("scegli le shortcut").size(40)

                    //padding([alto,sx,basso,dx])

                ])

                .width(Fill)
                .height(Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
            ]
                .spacing(20),
        )
            .into()
    }

    type Message = ShortcutMessage;

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}

