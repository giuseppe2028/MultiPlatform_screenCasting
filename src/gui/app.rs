use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use xcap::Monitor;

use super::component::caster_streaming;
use crate::controller::app_controller::AppController;
use crate::controller::receiver_controller::ReceiverController;
use crate::gui::component::caster_settings;
use crate::gui::component::caster_settings::CasterSettings;
use crate::gui::component::caster_streaming::{CasterStreaming, MessageUpdate};
use crate::gui::component::connection::Connection;
use crate::gui::component::home::Home;
use crate::gui::component::home::Role;
use crate::gui::component::receiver_ip;
use crate::gui::component::receiver_ip::ReceiverIp;
use crate::gui::component::receiver_streaming;
use crate::gui::component::receiver_streaming::ReceiverStreaming;
use crate::gui::component::window_part_screen::{MessagePress, WindowPartScreen};
use crate::gui::component::{home, Component};
use crate::gui::theme::widget::Element;
use crate::gui::theme::Theme;
use iced::time::{self, Duration};
use iced::{executor, Application, Command, Subscription};
use scap::capturer::Options;
use xcap::image::RgbaImage;

pub struct App {
    current_page: Page,
    home: Home,
    connection: Connection,
    receiver_ip: ReceiverIp,
    caster_settings: CasterSettings,
    receiver_streamimg: ReceiverStreaming,
    caster_streaming: CasterStreaming,
    controller: Controller,
    windows_part_screen: WindowPartScreen,
    sender: Sender<RgbaImage>,
}
enum Controller {
    ReceiverController(ReceiverController),
    CasterContoller(AppController),
    NotDefined,
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
    WindowPartScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartSharing, /*(connection::Message)*/
    RoleChosen(home::Message),
    ReceiverSharing(String),
    ReceiverInputIp(receiver_ip::Message),
    SetSettingsCaster(caster_settings::Window),
    Back(Page),
    StartRecording(receiver_streaming::Message),
    TogglerChanged(caster_streaming::MessageUpdate),
    SelectDisplay(Monitor),
    Close,
    UpdateScreen,
    StartPartialSharing(f32, f32, f64, f64),
    AreaSelectedFirst,
    AreaSelectedSecond,
    CursorMoved(f32, f32),
    StopStreaming,
    None,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (sender, receiver) = channel::<RgbaImage>();

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
                    available_displays: Monitor::all().unwrap(), //controller.get_available_displays(),
                    selected_display: Monitor::all().unwrap().get(0).unwrap().clone(), //controller.get_available_displays().get(0).unwrap().clone(),
                }, //implementare un metodo backend da chiamare per trovare gli screen
                caster_streaming: CasterStreaming {
                    toggler: false,
                    receiver: Arc::new(Mutex::new(receiver)),
                    frame_to_update: Arc::new(Mutex::new(None)),
                    measures: (0, 0),
                },
                windows_part_screen: WindowPartScreen {
                    screenshot: None,
                    coordinate: [(0.0, 0.0); 2],
                    cursor_position: (0.0, 0.0),
                    screen_dimension: (0.0, 0.0),
                    measures: (0, 0),
                },
                controller: Controller::NotDefined, //definisco il controller quando scelgo il ruolo
                sender,
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
            Message::RoleChosen(role) => match role {
                home::Message::ChosenRole(role) => match role {
                    Role::Caster => {
                        let socket = crate::socket::socket::CasterSocket::new("127.0.0.1:8000"); //"0.0.0.0:8000"
                        self.controller = Controller::CasterContoller(AppController::new(
                            Monitor::all().unwrap().get(0).unwrap().clone(),
                            self.sender.clone(),
                            socket,
                        ));
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
                self.current_page = Page::CasterStreaming; //dovrei mettere qui una pagina di caricamento
                if let Controller::CasterContoller(caster) = &mut self.controller {
                    caster.listens_for_receivers(); //dato che questa funzione finché non si collega almeno yun bro non si ferma e poi quando esce mettere la pagina CasterStreaming
                    caster.start_sharing();
                    self.caster_streaming.measures = caster.get_measures();
                }
                Command::none()
            }
            Message::ReceiverSharing(ip_caster) => {
                let socket = crate::socket::socket::ReceiverSocket::new(
                    "127.0.0.1:8001",
                    &format!("{}:8000", ip_caster),
                ); //TODO: calcolare la porta disponibile, tenere traccia del n di receiver
                self.controller = Controller::ReceiverController(ReceiverController::new(socket));
                self.current_page = Page::ReceiverStreaming;
                if let Controller::ReceiverController(receiver) = &mut self.controller {
                    receiver.register();
                    receiver.start_receiving();
                }
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
                        //self.controller.clean_options(); // MODIFICARE PER GIUSEPPE
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
                    Page::WindowPartScreen => {
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
                    }
                    caster_settings::Window::Area => {
                        if let Controller::CasterContoller(caster) = &mut self.controller {
                            self.windows_part_screen.screenshot = Some(caster.take_screenshot());
                            self.windows_part_screen.measures = caster.get_measures();
                        }
                        self.current_page = Page::WindowPartScreen
                    }
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
                if let Controller::CasterContoller(caster) = &mut self.controller {
                    caster.set_display(display.clone());
                }
                let _ = self
                    .caster_settings
                    .update(caster_settings::Message::SelectDisplay(display));
                Command::none()
            }
            Message::Close => {
                if let Controller::CasterContoller(caster) = &mut self.controller {
                    caster.stop_streaming();
                }
                else if let Controller::ReceiverController(receiver) = &mut self.controller {
                    receiver.stop_streaming();
                }
                //self.controller.clean_options(); DA FARE PER PEPPINO
                self.current_page = Page::Home;
                //TODO fare in modo di tornare alla schermata precedente
                Command::none()
            }
            Message::UpdateScreen => {
                // Scope the lock to limit the immutable borrow of `self.caster_streaming.receiver`
                let frame = {
                    if let Ok(receiver) = self.caster_streaming.receiver.lock() {
                        receiver.try_recv().ok()
                    } else {
                        None
                    }
                };

                // Now that the lock is dropped, we can mutate `self.caster_streaming`
                if let Some(frame) = frame {
                    let _ = self.caster_streaming.update(MessageUpdate::NewFrame(frame));
                }

                Command::none()
            }
            Message::StartPartialSharing(x, y, start_x, start_y) => {
                self.current_page = Page::CasterStreaming;
                // let target = self.controller.option.target.clone(); PEPPINO
                //calcolo la x rapportata ai valori dello schermo:
                //let (x,y) = get_screen_scaled(x,get_target_dimensions(&target.unwrap())); PEPPINO
                // self.controller.set_coordinates(x as f64, y as f64,start_x,start_y); SEMPRE PEPPINO C'E' PROPRIO LA STRUTTURA WINDOW IN XCAP
                if let Controller::CasterContoller(caster) = &mut self.controller {
                    caster.start_sharing();
                    self.caster_streaming.measures = caster.get_measures();
                }
                Command::none()
            }
            Message::AreaSelectedFirst => {
                let _ = self.windows_part_screen.update(MessagePress::FirstPress);
                Command::none()
            }
            Message::AreaSelectedSecond => {
                let _ = self.windows_part_screen.update(MessagePress::SecondPress);
                Command::none()
            }
            Message::CursorMoved(x, y) => {
                let _ = self
                    .windows_part_screen
                    .update(MessagePress::CursorMoved(x, y));
                Command::none()
            }
            Message::StopStreaming => {
                if let Controller::CasterContoller(caster) = &mut self.controller {
                    if caster.is_just_stopped {
                        caster.start_sharing();
                        caster.set_is_just_stopped(false);
                    } else {
                        caster.stop_streaming();
                        caster.set_is_just_stopped(true);
                    }
                }
                Command::none()
            }
            Message::None => Command::none(),
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
            Page::WindowPartScreen => self.windows_part_screen.view(),
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        // Always refresh the screen
        let mut subscriptions =
            vec![time::every(Duration::from_millis(16)).map(|_| Message::UpdateScreen)];

        // Add `WindowPartScreen`'s subscription only if on `Page::WindowPartScreen`
        if let Page::WindowPartScreen = self.current_page {
            subscriptions.push(
                self.windows_part_screen
                    .subscription()
                    .map(MessagePress::into),
            );
        }
        if let Page::CasterStreaming = self.current_page {
            subscriptions.push(
                self.caster_streaming
                    .subscription()
                    .map(MessageUpdate::into),
            );
        }

        Subscription::batch(subscriptions)
    }
}
