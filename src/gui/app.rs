use iced::{executor, Application, Command};

use crate::gui::component::caster_settings;
use crate::gui::component::caster_settings::CasterSettings;
use crate::gui::component::caster_streaming::CasterStreaming;
use crate::gui::component::connection::Connection;
use crate::gui::component::home::Home;
use crate::gui::component::home::Role;
use crate::gui::component::receiver_ip;
use crate::gui::component::receiver_ip::ReceiverIp;
use crate::gui::component::receiver_streaming;
use crate::gui::component::receiver_streaming::ReceiverStreaming;
use crate::gui::component::{home, Component};
use crate::gui::theme::widget::Element;
use crate::gui::theme::Theme;

use super::component::caster_streaming;

pub struct App {
    current_page: Page,
    home: Home,
    connection: Connection,
    receiver_ip: ReceiverIp,
    caster_settings: CasterSettings,
    receiver_streamimg: ReceiverStreaming,
    caster_streaming: CasterStreaming,
}

#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Selection,
    Connection,
    ReceiverIp,
    CasterSettings,
    ReceiverStreaming,
    CasterStreaming,
}

#[derive(Debug, Clone)]
pub enum Message {
    Router(Page),
    StartSharing, /*(connection::Message)*/
    RoleChosen(home::Message),
    ReceiverSharing(String),
    ReceiverInputIp(receiver_ip::Message),
    SetSettingsCaster(caster_settings::Message),
    Back(Page),
    StartRecording(receiver_streaming::Message),
    TogglerChanged(caster_streaming::Message),
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
                home: Home {},
                connection: Connection {
                    ip_address: "127.0.0.1".to_string(),
                },
                receiver_ip: ReceiverIp {
                    indirizzo_ip: "".to_string(),
                },
                receiver_streamimg: ReceiverStreaming { recording: false },
                caster_settings: CasterSettings {},
                caster_streaming: CasterStreaming { toggler: false },
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
            }
            Message::RoleChosen(role) => match role {
                home::Message::ChosenRole(role) => match role {
                    Role::Caster => {
                        self.current_page = Page::CasterSettings;
                        Command::none()
                    }
                    Role::Receiver => {
                        self.current_page = Page::Selection;
                        Command::none()
                    }
                },
            },
            Message::StartSharing => {
                self.current_page = Page::CasterStreaming;
                //aggiungere funzione backend
                Command::none()
            }
            Message::ReceiverSharing(_) => {
                self.current_page = Page::ReceiverStreaming;
                //aggiungere funzione backend
                Command::none()
            }
            Message::Back(page) => {
                match page {
                    Page::Home => {}
                    Page::Selection => {
                        self.current_page = Page::Home;
                    }
                    Page::Connection => {
                        self.current_page = Page::CasterSettings;
                    }
                    Page::ReceiverIp => {
                        self.current_page = Page::Home;
                    }
                    Page::ReceiverStreaming => {
                        self.current_page = Page::Home;
                    }
                    Page::CasterSettings => {
                        self.current_page = Page::Home;
                    }
                    Page::CasterStreaming => {
                        self.current_page = Page::Home;
                    }
                }
                Command::none()
            }
            Message::ReceiverInputIp(message) => {
                let _ = self.receiver_ip.update(message);
                Command::none()
            }
            Message::SetSettingsCaster(_) => {
                //prendere tutte le informazioni e passarle al backend per come deve avvenire lo streaming
                self.current_page = Page::Connection;
                Command::none()
            }
            Message::StartRecording(message) => {
                //funzioni backend
                let _ = self.receiver_streamimg.update(message);
                Command::none()
            }
            Message::TogglerChanged(message) => {
                let _ = self.caster_streaming.update(message);
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Home => self.home.view(),
            Page::Selection => self.receiver_ip.view(),
            Page::Connection => self.connection.view(),
            Page::ReceiverIp => self.receiver_ip.view(),
            Page::ReceiverStreaming => self.receiver_streamimg.view(),
            Page::CasterSettings => self.caster_settings.view(),
            Page::CasterStreaming => self.caster_streaming.view(),
        }
    }
}
