use super::component::caster_streaming;
use iced::multi_window::Application;
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
use crate::model::shortcut::{from_key_to_string, ShortcutController};
use crate::socket::socket::{CasterSocket, ReceiverSocket};
use crate::utils::utils::get_screen_scaled;
use iced::time::{self, Duration};
use iced::{executor, Command, Subscription, font, window, Size, Color, Background, Border, Length};
use std::sync::{Arc, RwLock};
use std::sync::atomic::Ordering;
use futures::FutureExt;
use iced::keyboard::Key;
use iced::widget::{container, Text};
use iced::widget::container::Appearance;
use iced::window::{Level, Position};
use iced::window::settings::PlatformSpecific;
use rand::Rng;
use tokio::sync::{mpsc::{channel, Sender}, Mutex};
use xcap::image::RgbaImage;
use xcap::Monitor;
use crate::gui::component::AnnotationToolsComponent::AnnotationTools;
use crate::gui::theme::container::Style;

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
    notification_rx: Option<tokio::sync::watch::Receiver<usize>>,
    second_window_id: Option<window::Id>,
    annotationTools: AnnotationTools
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
    FontLoaded(Result<(), font::Error>),
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
                    message: "".to_string(),
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
                    warning_message: false,
                    viewrs: Arc::new(RwLock::new(0)),
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
                    manage_transmission: from_key_to_string(
                        shortcut_controller.get_manage_trasmition_shortcut(),
                    )
                        .to_string(),
                    blancking_screen: from_key_to_string(
                        shortcut_controller.get_blanking_screen_shortcut(),
                    )
                        .to_string(),
                    terminate_session: from_key_to_string(
                        shortcut_controller.get_terminate_session_shortcut(),
                    )
                        .to_string(),
                },
                shortcut_controller,
                notification_rx: None,
                second_window_id: None,
                annotationTools: AnnotationTools {},
            },
            Command::batch(vec![
                font::load(include_bytes!("../../resources/home-icon.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../../resources/Barlow-Regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../../resources/Barlow-Bold.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self, window_id: window::Id) -> String {
        if window_id == window::Id::MAIN {
            String::from("MultiPlatform ScreenSharing")
        } else if Some(window_id) == self.second_window_id {
            "Second Window".to_owned()
        } else {
            unreachable!("invalid window")
        }
    }

    fn theme(&self, window_id: window::Id) -> Self::Theme {
        if window_id == window::Id::MAIN{
            Theme::Dark
        }else if Some(window_id) == self.second_window_id {
            Theme::Transparent
        }else{
            unreachable!("invalid window")
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::FontLoaded(result) => {
                if let Err(error) = result {
                    println!("{:?}", error);
                } else {
                    println!("Font caricato con successo");
                }
                Command::none()
            }
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
                        self.current_page = Page::ReceiverIp;
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
                let (notification_tx, notification_rx) = tokio::sync::watch::channel(0);
                self.notification_rx = Some(notification_rx);
                Command::perform(
                    async move {
                        println!("Creata nuova socket caster");
                        let socket = crate::socket::socket::CasterSocket::new(
                            "127.0.0.1:8000",
                            notification_tx,
                        )
                            .await;

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
                    let mut rng = rand::thread_rng();
                    let num: u8 = rng.gen_range(1..10); // Genera un numero casuale tra 0 e 9
                    Command::perform(
                        async move {
                            let socket = crate::socket::socket::ReceiverSocket::new(
                                &format!("127.0.0.1:800{}", num),
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
                    if let Controller::ReceiverController(receiver) = &mut self.controller {
                        match receiver.register() {
                            Ok(_) => {
                                self.current_page = Page::ReceiverStreaming;
                                receiver.start_receiving();
                            }
                            Err(message) => {
                                self.receiver_ip.message = message;
                                self.controller = Controller::NotDefined;
                            }
                        }
                    }
                    Command::none()
                }
            }
            Message::ReceiverControllerCreated(socket, sender, page) => {
                self.controller =
                    Controller::ReceiverController(ReceiverController::new(sender, socket));
                if let Controller::ReceiverController(receiver) = &mut self.controller {
                    match receiver.register() {
                        Ok(_) => {
                            self.current_page = page;
                            receiver.start_receiving();
                        }
                        Err(message) => {
                            self.receiver_ip.message = message;
                            self.controller = Controller::NotDefined;
                        }
                    }
                }
                Command::none()
            }
            Message::Back(page) => {
                self.current_page = match page {
                    Page::Home => Page::Home,
                    Page::Connection => Page::CasterSettings,
                    Page::ReceiverIp => {
                        self.receiver_ip.message = "".to_string();
                        Page::Home
                    }
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
                        if receiver_controller.is_recording.load(Ordering::Relaxed) {
                            receiver_controller.stop_recording();
                        }
                        receiver_controller.is_recording.store(!receiver_controller.is_recording.load(Ordering::Relaxed), Ordering::Relaxed);
                        print!("sono in start recording {}", receiver_controller.is_recording.load(Ordering::Relaxed));
                        // Se `self.controller` è di tipo `ReceiverController`, fai qualcosa
                        let _ = self.receiver_streaming.update(message);
                        Command::none()
                    }
                    _ => {
                        let _ = self.receiver_streaming.update(message);
                        Command::none()
                    }
                }
            }
            Message::TogglerChanged(message) => {
                println!("open");
                let (second_window_id, command) = window::spawn::<Message>(window::Settings {
                    size: Size::new(1024.0, 768.0),
                    position: Position::default(),
                    min_size: None,
                    max_size: None,
                    visible: true,
                    resizable: true,
                    decorations: true,
                    transparent: true,
                    level: Level::default(),
                    icon: None,
                    exit_on_close_request: true,
                    platform_specific: PlatformSpecific::default(),
                });
                self.second_window_id = Some(second_window_id);

                let _ = self.caster_streaming.update(message);
                command
            }
            Message::KeyShortcut(key_code) => {
                println!("SOno in in key {:?}", key_code);
                if let Controller::CasterController(caster) = &mut self.controller {
                    let key_code = from_key_to_string(key_code);
                    println!(" Key_code {:?}", key_code);
                    println!("SOno in in self.shorcut {:?}", self.shortcut_screen.blancking_screen);
                    if self.shortcut_screen.blancking_screen == key_code {
                        self.caster_streaming.warning_message =
                            !self.caster_streaming.warning_message;
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
                    receiver.unregister();
                    receiver.close_streaming();
                    *self.receiver_streaming.frame_to_update.blocking_lock() = None;
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
                        // if self.receiver_streaming.recording {
                        controller.start_recording(frame.clone());
                        //}
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
                let (notification_tx, notification_rx) = tokio::sync::watch::channel(0);
                self.notification_rx = Some(notification_rx);
                //creo la caster socket
                Command::perform(
                    async move {
                        println!("Creata nuova socket caster");
                        let socket = crate::socket::socket::CasterSocket::new(
                            "127.0.0.1:8000",
                            notification_tx,
                        )
                            .await;

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
                            if let Some(notification_rx) = self.notification_rx.clone() {
                                let viewrs_clone = self.caster_streaming.viewrs.clone();
                                tokio::spawn(async move {
                                    let mut notification_rx = notification_rx; // Clona il ricevitore per usarlo nel task
                                    while notification_rx.changed().await.is_ok() {
                                        // Ricevi il valore aggiornato
                                        let viewers = *notification_rx.borrow();
                                        *viewrs_clone.write().unwrap() = viewers;
                                        println!(
                                            "Numero di visualizzatori aggiornato: {}",
                                            viewers
                                        );
                                    }
                                    println!("TERMINO...");
                                });
                            } else {
                                eprintln!("Errore: notification_rx non è inizializzato!");
                            }
                            //caster.listens_for_receivers(); non serve più
                            caster.start_sharing_partial_sharing([
                                start_screen_scaled,
                                (screen_scaled),
                            ]);
                            self.current_page = page;
                        } else {
                            eprintln!("ERRORE NEL SETTAGGIO DELLA SOCKET");
                        }
                    }
                    Modality::Full => {
                        if let Controller::CasterController(caster) = &mut self.controller {
                            caster.set_socket(caster_socket);
                            if let Some(notification_rx) = self.notification_rx.clone() {
                                let viewrs_clone = self.caster_streaming.viewrs.clone();
                                tokio::spawn(async move {
                                    let mut notification_rx = notification_rx; // Clona il ricevitore per usarlo nel task
                                    while notification_rx.changed().await.is_ok() {
                                        // Ricevi il valore aggiornato
                                        let viewers = *notification_rx.borrow();
                                        *viewrs_clone.write().unwrap() = viewers;
                                        println!(
                                            "Numero di visualizzatori aggiornato: {}",
                                            viewers
                                        );
                                    }
                                    println!("TERMINO...");
                                });
                            } else {
                                eprintln!("Errore: notification_rx non è inizializzato!");
                            }
                            caster.start_sharing();
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

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if window_id == window::Id::MAIN {
            match self.current_page {
                Page::Home => self.home.view(),
                Page::Connection => self.connection.view(),
                Page::ReceiverIp => self.receiver_ip.view(),
                Page::ReceiverStreaming => self.receiver_streaming.view(),
                Page::CasterSettings => self.caster_settings.view(),
                Page::CasterStreaming => self.caster_streaming.view(),
                Page::WindowPartScreen => self.windows_part_screen.view(),
                Page::Shortcut => self.shortcut_screen.view(),
            }
        }else if Some(window_id) == self.second_window_id{
            println!("{:?}",self.current_page);
            match self.current_page {
                Page::CasterStreaming => self.annotationTools.view(),
                _ => {
                    unreachable!("To implement")
                }
            }
        }else{
            unreachable!("invalid window")
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



struct TransparentStyle;

impl container::StyleSheet for TransparentStyle {
    type Style = Style;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance{
            text_color: None,
            background: None,
            border: Border{
            color: Color::BLACK,
            width: 2.0,
            radius: Default::default(),
        },
            shadow: Default::default(),
        }
    }
}
