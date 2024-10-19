use std::process::Stdio;

use crate::controller::AppController::AppController;
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
use iced::{executor, Application, Command};
use scap::capturer::Options;
use super::component::caster_streaming;

pub struct App {
    current_page: Page,
    home: Home,
    connection: Connection,
    receiver_ip: ReceiverIp,
    caster_settings: CasterSettings,
    receiver_streamimg: ReceiverStreaming,
    caster_streaming: CasterStreaming,
    controller: AppController,
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
    SetSettingsCaster(caster_settings::Window),
    Back(Page),
    StartRecording(receiver_streaming::Message),
    TogglerChanged(caster_streaming::Message),
    SelectDisplay(scap::targets::Display),
    Close
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {

        let default_opt = Options {
            fps: 120,
            show_cursor: true,
            show_highlight: true,
            excluded_targets: None,
            target: None,
            output_type: scap::frame::FrameType::RGB,
            output_resolution: scap::capturer::Resolution::_1080p,  //USARE LIBREARIA CHE TROVA LA RISOLUZIONE DELLO SCHERMO

            ..Default::default()
        };



        //kill(Pid::from_raw(child.id() as i32), Signal::SIGKILL).expect("Errore nell'invio del segnale");
        //let childStdin = child.stdin.as_mut().unwrap();

        let mut controller = AppController::new(default_opt);
        controller.set_display(controller.get_available_displays().get(0).unwrap().clone());

        (
            Self {
                current_page: Page::Home,
                home: Home {},
                connection: Connection {
                    ip_address: "".to_string(),
                },
                receiver_ip: ReceiverIp {
                    indirizzo_ip: "".to_string(),
                },
                receiver_streamimg: ReceiverStreaming { recording: false },
                caster_settings: CasterSettings {
                    available_displays: controller.get_available_displays(),
                    selected_display: controller.get_available_displays().get(0).unwrap().clone(),
                }, //implementare un metodo backend da chiamare per trovare gli screen
                caster_streaming: CasterStreaming { toggler: false },
                controller: controller,

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
                print!("Bottone Premuto");
                self.current_page = Page::CasterStreaming;
                self.controller.start_sharing();
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
            //TODO adjust self.connection.ip_address = "".parse().unwrap();
            Message::SetSettingsCaster(message) => {
                self.connection.ip_address = "127.0.0.1".parse().unwrap(); //richiamare la funzione che si mette ad aspettare almeno una connessione e restituisce l'indirizzo ip del caster

                match message {
                    caster_settings::Window::FullScreen => {
                        self.current_page = Page::Connection;
                        //settare la risoluzione
                    },
                    caster_settings::Window::Area { x, y } => {
                        todo!()
                    },
                }

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
            }
            Message::SelectDisplay(display) => {
                //azione di quando sceglie quale schermo condividere
                self.controller.set_display(display.clone());
                let _ = self.caster_settings.update( caster_settings::Message::SelectDisplay(display));
                Command::none()
            }
            Message::Close=>{
                println!("ciao");
                self.controller.stop_recording();
                self.current_page = Page::Home;
                    //TODO fare in modo di tornare alla schermata precedente
                Command::none()
            }
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
