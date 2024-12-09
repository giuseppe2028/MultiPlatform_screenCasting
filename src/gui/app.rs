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
use crate::gui::component::receiver_streaming::{ReceiverStreaming, UpdateMessage};
use crate::gui::component::shorcut::{Shortcut, ShortcutMessage, Shortcuts};
use crate::gui::component::window_part_screen::{MessagePress, WindowPartScreen};
use crate::gui::component::{home, Component};
use crate::gui::theme::widget::Element;
use crate::gui::theme::Theme;
use crate::model::Shortcut::{from_key_to_string, ShortcutController};
use crate::socket::socket::{CasterSocket, ReceiverSocket};
use crate::utils::utils::get_screen_scaled;
use iced::time::{self, Duration};
use iced::{executor, Application, Command, Subscription};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use iced::keyboard::Key;
use iced::widget::image;
use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};
use xcap::image::RgbaImage;
use xcap::Monitor;

pub struct App {
    current_page: Page,
    home: Home,
    connection: Connection,
    receiver_ip: ReceiverIp,
    caster_settings: CasterSettings,
    receiver_streaming: ReceiverStreaming,
    caster_streaming: CasterStreaming,
    controller: Controller,
    windows_part_screen: WindowPartScreen,
    sender_caster: Sender<RgbaImage>,
    sender_receiver: Sender<RgbaImage>,
    shortcut_screen: Shortcut,
    shortcut_controller: ShortcutController,
}

enum Controller {
    ReceiverController(ReceiverController),
    CasterController(AppController),
    NotDefined,
}

#[derive(Debug, Clone)]
pub enum Modality {
    Partial(f32, f32, f64, f64),
    Full,
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
    Shortcut,
}

#[derive(Debug, Clone)]
pub enum Message {
    Route(Page),
    StartSharing,
    RoleChosen(home::Message),
    ReceiverSharing(String),
    ReceiverInputIp(receiver_ip::Message),
    SetSettingsCaster(caster_settings::Window),
    Back(Page),
    StartRecording(receiver_streaming::UpdateMessage),
    TogglerChanged(caster_streaming::MessageUpdate),
    KeyShortcut(Key),
    SelectDisplay(Monitor),
    Close,
    UpdateScreen,
    StartPartialSharing(f32, f32, f64, f64),
    AreaSelectedFirst,
    AreaSelectedSecond,
    CursorMoved(f32, f32),
    StopStreaming,
    None,
    SetCasterSocket(CasterSocket, Page, Modality),
    ReceiverControllerCreated(ReceiverSocket, Sender<RgbaImage>, Page),
    ChosenShortcuts(Shortcuts),
    Blanking,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (sender_caster, receiver_caster) = channel::<RgbaImage>(32); // Define buffer size
        let (sender_receiver, receiver_receiver) = channel::<RgbaImage>(32); // Define buffer size
        let shortcut_controller = ShortcutController::new_from_file();
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
                receiver_streaming: ReceiverStreaming {
                    recording: false,
                    receiver: Arc::new(Mutex::new(receiver_receiver)),
                    frame_to_update: Arc::new(Mutex::new(None)),
                    is_loading: true,
                },
                caster_settings: CasterSettings {
                    available_displays: Monitor::all().unwrap(),
                    selected_display: Monitor::all().unwrap().get(0).unwrap().clone(),
                },
                caster_streaming: CasterStreaming {
                    toggler: false,
                    receiver: Arc::new(Mutex::new(receiver_caster)),
                    frame_to_update: Arc::new(Mutex::new(None)),
                    measures: (0, 0),
                    is_loading: true,
                    shortcut: ShortcutController::new_from_file(),
                    warning_message: false,
                },
                windows_part_screen: WindowPartScreen {
                    screenshot: None,
                    coordinate: [(0.0, 0.0); 2],
                    cursor_position: (0.0, 0.0),
                    screen_dimension: (0.0, 0.0),
                    measures: (0, 0),
                },
                controller: Controller::NotDefined,
                sender_caster,
                sender_receiver,

                shortcut_screen: Shortcut {
                    // @giuseppe2028 metti le funzioni di default
                    manage_transmission: from_key_to_string(
                        shortcut_controller.get_manage_trasmition_shortcut(),
                    )
                    .to_string(),
                    // @giuseppe2028 metti le funzioni di default
                    blancking_screen: from_key_to_string(
                        shortcut_controller.get_blanking_screen_shortcut(),
                    )
                    .to_string(),
                    // @giuseppe2028 metti le funzioni di default
                    terminate_session: from_key_to_string(
                        shortcut_controller.get_terminate_session_shortcut(),
                    )
                    .to_string(),
                    err_key_set: false,
                },
                shortcut_controller,
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
                        self.controller = Controller::CasterController(AppController::new(
                            Monitor::all().unwrap().get(0).unwrap().clone(),
                            self.sender_caster.clone(),
                            None,
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
            Message::Route(page) => {
                self.current_page = page;
                Command::none()
            }
            Message::StartSharing => {
                //devo creare solo la socket
                Command::perform(
                    async move {
                        println!("Creata nuova socket caster");
                        let socket =
                            crate::socket::socket::CasterSocket::new("127.0.0.1:8000").await;

                        let page = Page::CasterStreaming;
                        (socket, page)
                    },
                    move |(socket, page)| Message::SetCasterSocket(socket, page, Modality::Full),
                )
            }
            Message::ReceiverSharing(ip_caster) => {
                //creo controller e socket insieme tanto il controller non mi serve prima per il receiver
                if let Controller::NotDefined = &mut self.controller {
                    let sender = self.sender_receiver.clone();
                    Command::perform(
                        async move {
                            let socket = crate::socket::socket::ReceiverSocket::new(
                                "127.0.0.1:8001",
                                &format!("{}:8000", ip_caster),
                            )
                            .await;
                            let page = Page::ReceiverStreaming;
                            (socket, sender, page)
                        },
                        move |(socket, sender, page)| {
                            // Once the operation is complete, send a "ControllerCreated" message
                            //self.controller = caster_controller;
                            Message::ReceiverControllerCreated(socket, sender, page)
                        },
                    )
                } else {
                    self.current_page = Page::ReceiverStreaming;
                    if let Controller::ReceiverController(receiver) = &mut self.controller {
                        receiver.register();
                        receiver.start_receiving();
                    }
                    Command::none()
                }
            }
            Message::ReceiverControllerCreated(socket, sender, page) => {
                self.controller =
                    Controller::ReceiverController(ReceiverController::new(sender, socket));
                self.current_page = page;
                if let Controller::ReceiverController(receiver) = &mut self.controller {
                    receiver.register();
                    receiver.start_receiving();
                }
                Command::none()
            }
            Message::Back(page) => {
                self.current_page = match page {
                    Page::Home => Page::Home,
                    Page::Selection => Page::Home,
                    Page::Connection => Page::CasterSettings,
                    Page::ReceiverIp => Page::Home,
                    Page::ReceiverStreaming => Page::Home,
                    Page::CasterSettings => Page::Home,
                    Page::CasterStreaming => Page::Home,
                    Page::WindowPartScreen => Page::Home,
                    Page::Shortcut => Page::Home,
                };
                Command::none()
            }
            Message::ReceiverInputIp(message) => {
                let _ = self.receiver_ip.update(message);
                Command::none()
            }
            Message::SetSettingsCaster(message) => {
                self.connection.ip_address = "127.0.0.1".parse().unwrap();
                match message {
                    caster_settings::Window::FullScreen => {
                        self.current_page = Page::Connection;
                    }
                    caster_settings::Window::Area => {
                        if let Controller::CasterController(caster) = &mut self.controller {
                            println!("CHIAMO LA FUNZIONE DELLO SCREENSHOT");
                            let frame = caster.take_screenshot();
                            self.windows_part_screen.screenshot = Some(frame);
                            self.windows_part_screen.measures = caster.get_measures();
                            self.current_page = Page::WindowPartScreen;
                        } else {
                            eprintln!("ERRORE");
                        }
                    }
                }

                Command::none()
            }
            Message::StartRecording(message) => {
                println!("entro dentro recording");
                match &self.controller {
                    Controller::ReceiverController(receiver_controller) => {
                        if receiver_controller.is_recording.load(Ordering::Relaxed){
                            receiver_controller.stop_recording();
                        }
                        receiver_controller.is_recording.store(!receiver_controller.is_recording.load(Ordering::Relaxed), Ordering::Relaxed);
                        print!("sono in start recording {}",receiver_controller.is_recording.load(Ordering::Relaxed));
                        // Se `self.controller` è di tipo `ReceiverController`, fai qualcosa
                        let _ = self.receiver_streaming.update(message);
                        Command::none()
                    },
                    _ => {
                        let _ = self.receiver_streaming.update(message);
                        Command::none()
                    }
                }
            }
            Message::TogglerChanged(message) => {
                let _ = self.caster_streaming.update(message);
                Command::none()
            }
            Message::KeyShortcut(key_code) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    let key_code = from_key_to_string(key_code);
                    if self.shortcut_screen.blancking_screen == key_code {
                        self.caster_streaming.warning_message = !self.caster_streaming.warning_message;
                        caster.blanking_streaming();
                    } else if self.shortcut_screen.terminate_session == key_code {
                        caster.stop_streaming();
                        //self.controller.clean_options(); DA FARE PER PEPPINO
                        self.current_page = Page::Home;
                    } else if self.shortcut_screen.manage_transmission == key_code {
                        if caster.is_just_stopped {
                            caster.start_sharing();
                            caster.set_is_just_stopped(false);
                        } else {
                            caster.stop_streaming();
                            caster.set_is_just_stopped(true);
                        }
                    }
                } else {
                    eprintln!("Dovrebbe essere impossibile arrivare qui SHORTCUT!!");
                }
                Command::none()
            }
            Message::Blanking => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    self.caster_streaming.warning_message = !self.caster_streaming.warning_message;
                    caster.blanking_streaming();
                } else {
                    eprintln!("NON DOVREBBE ENTRARE MAI QUI..BLANKING");
                }

                //aggiungere logica server
                Command::none()
            }
            Message::SelectDisplay(display) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster.set_display(display.clone());
                } else {
                    eprintln!("ERRORE NELLA SELEZIONE DELLO SCHERMO DA CONDIVIDERE");
                }
                let _ = self
                    .caster_settings
                    .update(caster_settings::Message::SelectDisplay(display));
                Command::none()
            }
            Message::Close => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster.close_streaming();
                    self.controller = Controller::NotDefined;
                } else if let Controller::ReceiverController(receiver) = &mut self.controller {
                    receiver.close_streaming();
                    self.controller = Controller::NotDefined;
                } else {
                    eprintln!("ERRORE NELLA CHIUSURA DELLA CONDIVISIONE SCHERMO");
                }
                self.current_page = Page::Home;
                Command::none()
            }
            Message::UpdateScreen => {
                match &self.controller {
                    Controller::ReceiverController(controller) => {
                        let frame = {
                            if let Ok(receiver) =
                                self.receiver_streaming.receiver.blocking_lock().try_recv()
                            {
                                receiver
                            } else {
                                return Command::none();
                            }
                        };
                            controller.start_recording(frame.clone());
                        let _ = self
                            .receiver_streaming
                            .update(UpdateMessage::NewFrame(frame));
                    }

                    Controller::CasterController(_) => {
                        let frame = {
                            if let Ok(receiver) =
                                self.caster_streaming.receiver.blocking_lock().try_recv()
                            {
                                receiver
                            } else {
                                return Command::none();
                            }
                        };
                        let _ = self.caster_streaming.update(MessageUpdate::NewFrame(frame));
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::StartPartialSharing(x, y, start_x, start_y) => {
                //creo la caster socket
                Command::perform(
                    async move {
                        println!("Creata nuova socket caster");
                        let socket =
                            crate::socket::socket::CasterSocket::new("127.0.0.1:8000").await;

                        let page = Page::CasterStreaming;
                        (socket, page)
                    },
                    move |(socket, page)| {
                        Message::SetCasterSocket(
                            socket,
                            page,
                            Modality::Partial(x, y, start_x, start_y),
                        )
                    },
                )
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
                if let Controller::CasterController(caster) = &mut self.controller {
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
            Message::SetCasterSocket(caster_socket, page, modality) => {
                match modality {
                    Modality::Partial(x, y, start_x, start_y) => {
                        if let Controller::CasterController(caster) = &mut self.controller {
                            caster.set_socket(caster_socket);
                            let screen_scaled = get_screen_scaled(
                                x as f64,
                                y as f64,
                                (
                                    caster.get_measures().0 as u64,
                                    caster.get_measures().1 as u64,
                                ),
                            );
                            let start_screen_scaled = get_screen_scaled(
                                start_x,
                                start_y,
                                (
                                    caster.get_measures().0 as u64,
                                    caster.get_measures().1 as u64,
                                ),
                            );
                            println!(
                                "x: {} y: {} start_x: {} start_y: {}",
                                x, y, screen_scaled.0, screen_scaled.1
                            );
                            caster.listens_for_receivers();
                            caster.start_sharing_partial_sharing([
                                start_screen_scaled,
                                (screen_scaled),
                            ]);
                            self.caster_streaming.measures = caster.get_measures();
                            self.current_page = page;
                        } else {
                            eprintln!("ERRORE NEL SETTAGGIO DELLA SOCKET");
                        }
                    }
                    Modality::Full => {
                        if let Controller::CasterController(caster) = &mut self.controller {
                            caster.set_socket(caster_socket);
                            caster.listens_for_receivers();
                            caster.start_sharing();
                            self.caster_streaming.measures = caster.get_measures();
                            self.current_page = page;
                        } else {
                            eprintln!("ERRORE NEL SETTAGGIO DELLA SOCKET")
                        }
                    }
                }
                Command::none()
            }
            Message::ChosenShortcuts(shortcuts) => {
                match shortcuts {
                    Shortcuts::ManageTransmission(key) => {
                        print!("entro manage");
                        self.shortcut_controller.set_manage_trasmition(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::ManageTransmission(key));
                    }
                    Shortcuts::BlanckingScreen(key) => {
                        print!("entro blanking");
                        self.shortcut_controller.set_blanking_screen(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::BlanckingScreen(key));
                    }
                    Shortcuts::TerminateSession(key) => {
                        print!("entro Trasmission");
                        self.shortcut_controller.set_terminate_session(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::TerminateSession(key));
                    }
                }
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
            Page::ReceiverStreaming => self.receiver_streaming.view(),
            Page::CasterSettings => self.caster_settings.view(),
            Page::CasterStreaming => self.caster_streaming.view(),
            Page::WindowPartScreen => self.windows_part_screen.view(),
            Page::Shortcut => self.shortcut_screen.view(),
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        // Always refresh the screen
        let mut subscriptions =
            vec![time::every(Duration::from_millis(10)).map(|_| Message::UpdateScreen)];

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
