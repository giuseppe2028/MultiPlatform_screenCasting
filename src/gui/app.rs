
use iced::{executor, Application, Command};

use crate::gui::component::connection::Connection;
use crate::gui::theme::Theme;
use crate::gui::component::home::Home;
use crate::gui::component::home::Role;
use crate::gui::theme::widget::Element;
use crate::gui::component::{home, Component};
use crate::gui::component::receiver_ip::ReceiverIp;

use super::component::receiver_ip;

pub struct App {
    current_page: Page,
    home: Home,
    connection: Connection,
    receiver_ip: ReceiverIp
}

#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Selection,
    Connection,
}

#[derive(Debug, Clone)]
pub enum Message {
    Router(Page),
    StartSharing/*(connection::Message)*/,
    RoleChosen(home::Message),
    ReceiverSharing(receiver_ip::Message)
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                current_page: Page::Home,
                home: Home{},
                connection: Connection{ ip_address: "127.0.0.1".to_string()},
                receiver_ip: ReceiverIp { indirizzo_ip: "".to_string() }
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MultiPlatform ScreenSharing")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Router(route) => {
                self.current_page = route;
                Command::none()
            },
            Message::RoleChosen(role) => {
                match role {
                    home::Message::ChosenRole(role) => {
                        match role {
                            Role::Caster => {
                                self.current_page = Page::Connection;
                                Command::none()
                            },
                            Role::Receiver => {
                                self.current_page = Page::Selection;
                                Command::none()
                            }
                        }
                    },
                }
            },
            Message::StartSharing => {
                //self.current_page = Page::Sharing
                //aggiungere funzione backend
                Command::none()
            },
            Message::ReceiverSharing(_) => {
                //self.current_page = Page::Connection
                //aggiungere funzione backend
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Home => {
                self.home.view()
            },
            Page::Selection => {
                self.receiver_ip.view()
            },
            Page::Connection => {
                self.connection.view()
            },
        }
    }
}
