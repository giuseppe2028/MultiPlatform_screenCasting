use super::component::caster_streaming;
use super::component::AnnotationToolsComponent::MessageAnnotation;
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
use iced::multi_window::Application;
use rand::{thread_rng, Rng};

use crate::gui::component::Annotation::Square::{
    CanvasWidget, LineCanva, Pending, RectangleCanva, Shape, Status,
};
use crate::gui::component::AnnotationToolsComponent::AnnotationTools;
use crate::gui::theme::container::Style;
use crate::gui::theme::Theme;
use crate::model::shortcut::{from_key_to_string, ShortcutController};
use crate::socket::socket::{CasterSocket, ReceiverSocket};
use crate::utils::utils::get_screen_scaled;
use iced::keyboard::Key;
use iced::time::{self, Duration};
use iced::widget::container;
use iced::widget::container::Appearance;
use iced::window::settings::PlatformSpecific;
use iced::window::{Level, Position};
use iced::{executor, font, window, Border, Color, Command, Point, Size, Subscription};
use local_ip_address::local_ip;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
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
    notification_rx: Option<tokio::sync::watch::Receiver<usize>>,
    second_window_id: Option<window::Id>,
    annotationTools: AnnotationTools,
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
    SetCasterSocket(CasterStreaming, CasterSocket, Page, Modality),
    ReceiverControllerCreated(ReceiverSocket, Sender<RgbaImage>, Page),
    ChosenShortcuts(Shortcuts),
    Blanking,
    PendingOne(Pending),
    PendingTwo(Pending),
    AddShape(Shape),
    ClearShape,
    SelectShape(Shape),
    ChooseColor,
    CancelColor,
    SubmitColor(Color),
    TextPressed(bool),
    TextCanvasChanged(String),
    SaveTextPosition(Point),
    SetColor,
    CloseRequested,
    ViewTools(CasterStreaming, MessageUpdate),
    ResetWindow,
    DoKeyShortcut(CasterStreaming, Key),
    StopStreamingReal(CasterStreaming),
    BlankingReal(CasterStreaming),
    RegistrationResult(Result<String, String>),
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
                    modality: Modality::Full,
                    stop: false,
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
                annotationTools: AnnotationTools {
                    canvas_widget: CanvasWidget::new(),
                    set_selected_annotation: false,
                    show_color_picker: false,
                    selected_color: Color::from_rgba8(0, 0, 0, 1.0),
                    window_id: None,
                },
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
            "Second Window".to_owned()
        }
    }

    fn theme(&self, window_id: window::Id) -> Self::Theme {
        if window_id == window::Id::MAIN {
            Theme::Dark
        } else if Some(window_id) == self.second_window_id {
            Theme::Transparent
        } else {
            Theme::Dark
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::CloseRequested => {
                // println!("Chiusa finestra secondaria");
                // Controllo se la finestra è valida prima di chiuderla
                let id = self.second_window_id.unwrap();
                let mut caster_streaming = self.caster_streaming.clone();
                Command::perform(
                    async move {
                        let result = window::close::<Message>(id); // Chiude la finestra solo se valida
                        result.actions();
                        caster_streaming.toggler = false;
                    },
                    |_| Message::ResetWindow,
                )
            }
            Message::ResetWindow => {
                self.second_window_id = None;
                Command::none()
            }
            Message::SetColor => {
                self.annotationTools.show_color_picker = true;
                Command::none()
            }
            Message::ChooseColor => {
                self.annotationTools.show_color_picker = true;
                //println!("sono in choose");
                Command::none()
            }
            Message::SubmitColor(color) => {
                //println!("Colore: {:?}", color);
                self.annotationTools.selected_color = color;
                self.annotationTools.show_color_picker = false;
                Command::none()
            }
            Message::PendingTwo(pending) => {
                if let Pending::Two { from: _, to } = pending {
                    self.annotationTools.canvas_widget.start_point = to;
                }
                Command::none()
            }
            Message::PendingOne(pending) => {
                if let Pending::One { from } = pending {
                    self.annotationTools.canvas_widget.start_point = from;
                }
                Command::none()
            }

            Message::FontLoaded(result) => {
                if let Err(error) = result {
                    println!("{:?}", error);
                } else {
                    //println!("Font caricato con successo");
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
                let caster_streaming = self.caster_streaming.clone();
                Command::perform(
                    async move {
                        println!("Creata nuova socket caster");
                        let caster_ip = local_ip().unwrap();
                        //println!("{:?}", caster_ip);
                        let socket = crate::socket::socket::CasterSocket::new(
                            &format!("{}:7879", caster_ip),
                            notification_tx,
                        )
                        .await;

                        let page = Page::CasterStreaming;
                        (caster_streaming.clone(), socket, page)
                    },
                    move |(caster, socket, page)| {
                        Message::SetCasterSocket(caster, socket, page, Modality::Full)
                    },
                )
            }
            Message::ReceiverSharing(ip_caster) => {
                //creo controller e socket insieme tanto il controller non mi serve prima per il receiver
                if let Controller::NotDefined = &mut self.controller {
                    let sender = self.sender_receiver.clone();
                    let mut rng = thread_rng();
                    let random = rng.gen_range(0..10);
                    Command::perform(
                        async move {
                            let receiver_ip = local_ip().unwrap();
                            //println!("{:?}", receiver_ip);
                            let socket = crate::socket::socket::ReceiverSocket::new(
                                &format!("{}:7880", receiver_ip),
                                &format!("{}:7879", ip_caster),
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
                    if let Controller::ReceiverController(receiver) = &self.controller {
                        let rec = receiver.clone();
                        Command::perform(async move { rec.register().await }, |result| {
                            Message::RegistrationResult(result)
                        })
                    } else {
                        Command::none()
                    }
                }
            }
            Message::RegistrationResult(result) => {
                if let Controller::ReceiverController(receiver) = &mut self.controller {
                    match result {
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
            Message::ReceiverControllerCreated(socket, sender, page) => {
                println!("dentro");
                self.controller =
                    Controller::ReceiverController(ReceiverController::new(sender, socket));
                if let Controller::ReceiverController(receiver) = &self.controller {
                    let rec = receiver.clone();
                    self.current_page = page;
                    Command::perform(async move { rec.register().await }, |result| {
                        Message::RegistrationResult(result)
                    })
                } else {
                    Command::none()
                }
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
                let _ = tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.receiver_ip.update(message))
                });
                Command::none()
    
            }
            Message::SetSettingsCaster(message) => {
                let caster_ip = local_ip().unwrap();
                self.connection.ip_address = caster_ip.to_string();
                match message {
                    caster_settings::Window::FullScreen => {
                        self.current_page = Page::Connection;
                    }
                    caster_settings::Window::Area => {
                        if let Controller::CasterController(caster) = &mut self.controller {
                            //println!("CHIAMO LA FUNZIONE DELLO SCREENSHOT");
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
                //println!("entro dentro recording");
                match &self.controller {
                    Controller::ReceiverController(receiver_controller) => {
                        if receiver_controller.is_recording.load(Ordering::Relaxed) {
                            receiver_controller.stop_recording();
                        }
                        receiver_controller.is_recording.store(
                            !receiver_controller.is_recording.load(Ordering::Relaxed),
                            Ordering::Relaxed,
                        );
                        /*print!(
                            "sono in start recording {}",
                            receiver_controller.is_recording.load(Ordering::Relaxed)
                        );*/
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
            Message::TogglerChanged(message) => match self.second_window_id {
                None => {
                    let caster_streaming = self.caster_streaming.clone();
                    Command::perform(
                        async move { (caster_streaming, message) },
                        |(caster_streaming, message)| Message::ViewTools(caster_streaming, message),
                    )
                }
                Some(_) => {
                    panic!("NON DOVREBBE ENTRARE QUI");
                }
            },
            Message::ViewTools(mut caster, message) => {
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
                self.annotationTools.window_id = Some(second_window_id);

                self.second_window_id = Some(second_window_id);
                let _ = Command::perform(
                    async move {
                        caster.update(message).await
                    },
                    |_| Message::None,
                );
                command
            }
            Message::KeyShortcut(key_code) => {
                //println!("SOno in in key {:?}",key_code);
                let caster_streaming = self.caster_streaming.clone();
                Command::perform(
                    async move { (caster_streaming, key_code) },
                    |(caster_streaming, key_code)| {
                        Message::DoKeyShortcut(caster_streaming, key_code)
                    },
                )
            }
            Message::DoKeyShortcut(mut caster_streaming, key_code) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    let mut cast = caster.clone();
                    let key_code = from_key_to_string(key_code);
                    //println!(" Key_code {:?}",key_code);
                    //println!("SOno in in self.shorcut {:?}", self.shortcut_screen.blancking_screen);
                    if self.shortcut_screen.blancking_screen == key_code {
                        caster_streaming.warning_message = !caster_streaming.warning_message;
                        caster.blanking_streaming();
                        Command::none()
                    } else if self.shortcut_screen.terminate_session == key_code {
                        self.current_page = Page::Home;
                        Command::perform(
                            async move {
                                cast.close_streaming().await
                            },
                            |_| Message::None,
                        )
                        //self.controller.clean_options(); DA FARE PER PEPPINO
                    } else if self.shortcut_screen.manage_transmission == key_code {
                        if caster.is_just_stopped {
                            match caster_streaming.modality {
                                Modality::Partial(x, y, start_x, start_y) => {
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
                                    caster.start_sharing_partial_sharing([
                                        start_screen_scaled,
                                        (screen_scaled),
                                    ]);
                                }
                                Modality::Full => {
                                    caster.start_sharing();
                                }
                            }
                            caster_streaming.stop = false;
                            caster.set_is_just_stopped(false);
                        } else {
                            caster.stop_streaming();
                            caster_streaming.stop = true;
                            caster.set_is_just_stopped(true);

                        }
                        Command::none()
                    }
                    else {
                        Command::none()
                    }
                } else {
                    panic!("Non dovrebbe entrare qui dentro");
                }
            }
            Message::Blanking => {
                let caster_streaming = self.caster_streaming.clone();
                Command::perform(async move { caster_streaming }, |caster| {
                    Message::BlankingReal(caster)
                })
            }
            Message::BlankingReal(mut caster_streaming) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster_streaming.warning_message = caster_streaming.warning_message;
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
                self.current_page = Page::Home;
                if let Controller::CasterController(caster) = &mut self.controller {
                    let mut cast = caster.clone();
                    self.controller = Controller::NotDefined;
                    Command::perform(
                        async move {
                            println!("chiudo");
                            cast.close_streaming().await
                        },
                        |_| Message::None,
                    )
                } else if let Controller::ReceiverController(receiver) = &mut self.controller {
                    let mut rec = receiver.clone();
                    let receiver_streaming = self.receiver_streaming.clone();
                    self.controller = Controller::NotDefined;
                    Command::perform(
                        async move {
                            rec.unregister().await;
                            println!("disconetto");
                            rec.close_streaming().await;
                            *receiver_streaming.frame_to_update.lock().await = None;
                        },
                        |_| Message::None,
                    )
                } else {
                    eprintln!("ERRORE NELLA CHIUSURA DELLA CONDIVISIONE SCHERMO");
                    Command::none()
                }
            }
            Message::UpdateScreen => {
                match &self.controller {
                    Controller::ReceiverController(controller) => {
                        let controller_cloned: ReceiverController = controller.clone(); //si potrebbe ottimizzare questa clone
                        let receiver_streaming = self.receiver_streaming.clone();
                        let mut receiver_streaming2 = self.receiver_streaming.clone();
                        Command::perform(
                            async move {
                                let mut resutl = receiver_streaming.receiver.lock().await;
                                // Perform async work here, like locking the receiver and handling the frame
                                if let Ok(receiver) = resutl.try_recv() {
                                    // Update the caster streaming
                                    if receiver_streaming.recording {
                                        controller_cloned.start_recording(receiver.clone());
                                    }
                                    let _ = receiver_streaming2
                                        .update(UpdateMessage::NewFrame(receiver))
                                        .await;
                                } else {
                                    // If the receiver couldn't be locked, handle the case (e.g., return early)
                                }
                            },
                            |_| {
                                // When the async work completes, you can return an appropriate message
                                // For example, you might return a message indicating a new frame has been processed.
                                Message::None // Your message type
                            },
                        )
                    }
                    Controller::CasterController(_) => {
                        let caster_streaming = self.caster_streaming.clone();
                        let mut caster_streaming2 = self.caster_streaming.clone();
                        Command::perform(
                            async move {
                                let mut resutl = caster_streaming.receiver.lock().await;
                                // Perform async work here, like locking the receiver and handling the frame
                                if let Ok(receiver) = resutl.try_recv() {
                                    // Update the caster streaming
                                    let _ = caster_streaming2
                                        .update(MessageUpdate::NewFrame(receiver))
                                        .await;
                                } else {
                                    // If the receiver couldn't be locked, handle the case (e.g., return early)
                                }
                            },
                            |_| {
                                // When the async work completes, you can return an appropriate message
                                // For example, you might return a message indicating a new frame has been processed.
                                Message::None // Your message type
                            },
                        )
                        /*
                        let frame = {
                            if let Ok(receiver) =
                                self.caster_streaming.receiver.lock().await
                            {
                                receiver
                            } else {
                                return Command::none();
                            }
                        };
                        let _ = self.caster_streaming.update(MessageUpdate::NewFrame(frame));*/
                    }
                    _ => Command::none(),
                }
            }
            Message::StartPartialSharing(x, y, start_x, start_y) => {
                let (notification_tx, notification_rx) = tokio::sync::watch::channel(0);
                self.notification_rx = Some(notification_rx);
                //creo la caster socket
                if let Controller::CasterController(_) = &mut self.controller {
                    let caster_streaming = self.caster_streaming.clone();
                    let _ = Command::perform(
                        async move {
                            //println!("Creata nuova socket caster");
                            let caster_ip = local_ip().unwrap();
                            //println!("{:?}", caster_ip);
                            let socket = crate::socket::socket::CasterSocket::new(
                                &format!("{}:7879", caster_ip),
                                notification_tx,
                            )
                            .await;
                            let page = Page::CasterStreaming;
                            (caster_streaming, socket, page)
                        },
                        move |(caster, socket, page)| {
                            Message::SetCasterSocket(
                                caster,
                                socket,
                                page,
                                Modality::Partial(x, y, start_x, start_y),
                            )
                        },
                    );
                } else {
                    eprintln!("Non dovrebbe entrare qua");
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
                let caster_streaming = self.caster_streaming.clone();
                Command::perform(async move { caster_streaming }, |caster| {
                    Message::StopStreamingReal(caster)
                })
            }
            Message::StopStreamingReal(mut caster_streaming) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    if caster.is_just_stopped {
                        match caster_streaming.modality {
                            Modality::Partial(x, y, start_x, start_y) => {
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
                                caster.start_sharing_partial_sharing([
                                    start_screen_scaled,
                                    (screen_scaled),
                                ]);
                            }
                            Modality::Full => {
                                caster.start_sharing();
                            }
                        }
                        caster_streaming.stop = false;
                        caster.set_is_just_stopped(false);
                    } else {
                        caster_streaming.stop = true;
                        caster.stop_streaming();
                        caster.set_is_just_stopped(true);
                    }
                }
                Command::none()
            }
            Message::None => Command::none(),
            Message::SetCasterSocket(mut caster_streaming, caster_socket, page, modality) => {
                caster_streaming.modality = modality.clone();
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
                            /* println!(
                                "x: {} y: {} start_x: {} start_y: {}",
                                x, y, screen_scaled.0, screen_scaled.1
                            );*/
                            if let Some(notification_rx) = self.notification_rx.clone() {
                                let viewrs_clone = caster_streaming.viewrs.clone();
                                tokio::spawn(async move {
                                    let mut notification_rx = notification_rx; // Clona il ricevitore per usarlo nel task
                                    while notification_rx.changed().await.is_ok() {
                                        // Ricevi il valore aggiornato
                                        let viewers = *notification_rx.borrow();
                                        *viewrs_clone.write().unwrap() = viewers;
                                        /*println!(
                                            "Numero di visualizzatori aggiornato: {}",
                                            viewers
                                        );*/
                                    }
                                    //println!("TERMINO...");
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
                                let viewrs_clone = caster_streaming.viewrs.clone();
                                tokio::spawn(async move {
                                    let mut notification_rx = notification_rx; // Clona il ricevitore per usarlo nel task
                                    while notification_rx.changed().await.is_ok() {
                                        // Ricevi il valore aggiornato
                                        let viewers = *notification_rx.borrow();
                                        *viewrs_clone.write().unwrap() = viewers;
                                        /*println!(
                                            "Numero di visualizzatori aggiornato: {}",
                                            viewers
                                        );*/
                                    }
                                    //println!("TERMINO...");
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
                        //print!("entro manage");
                        self.shortcut_controller.set_manage_trasmition(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::ManageTransmission(key));
                    }
                    Shortcuts::BlanckingScreen(key) => {
                        //print!("entro blanking");
                        self.shortcut_controller.set_blanking_screen(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::BlanckingScreen(key));
                    }
                    Shortcuts::TerminateSession(key) => {
                        //print!("entro Trasmission");
                        self.shortcut_controller.set_terminate_session(&key);
                        let _ = self
                            .shortcut_screen
                            .update(ShortcutMessage::TerminateSession(key));
                    }
                }
                Command::none()
            }
            Message::AddShape(shape) => {
                self.annotationTools.canvas_widget.shapes.push(shape);
                /*  println!(
                    "SONO O NON SONO IN Add shape {:?}",
                    self.annotationTools.canvas_widget.shapes
                );*/
                self.annotationTools.canvas_widget.cache.clear(); // Forza il ridisegno
                Command::none()
            }
            Message::SelectShape(shape) => {
                self.annotationTools.set_selected_annotation =
                    !self.annotationTools.set_selected_annotation;
                //print!("Hai scelto il rettangolo");
                match shape {
                    Shape::Line(_) => {
                        self.annotationTools.canvas_widget.selected_shape =
                            Some(Shape::Line(LineCanva {
                                starting_point: Default::default(),
                                ending_point: Default::default(),
                            }));
                        Command::none()
                    }
                    Shape::Rectangle(_) => {
                        //print!("Hai scelto il rettangolo1");
                        self.annotationTools.canvas_widget.selected_shape =
                            Some(Shape::Rectangle(RectangleCanva {
                                startPoint: Default::default(),
                                width: 0.0,
                                height: 0.0,
                            }));
                        Command::none()
                    }
                    Shape::Circle(circle) => {
                        //print!("Hai scelto il Cerchio");
                        self.annotationTools.canvas_widget.selected_shape =
                            Some(Shape::Circle(circle));
                        Command::none()
                    }
                    Shape::Arrow(arrow) => {
                        self.annotationTools.canvas_widget.selected_shape =
                            Some(Shape::Arrow(arrow));
                        Command::none()
                    }
                }
            }
            Message::ClearShape => {
                self.annotationTools.canvas_widget.shapes.clear();
                self.annotationTools.canvas_widget.all_text_selected.clear();
                self.annotationTools.canvas_widget.cache.clear();
                Command::none()
            }
            Message::TextPressed(condition) => {
                if !condition {
                    self.annotationTools
                        .canvas_widget
                        .all_text_selected
                        .push(self.annotationTools.canvas_widget.textSelected.clone());
                    self.annotationTools.canvas_widget.cache.clear();
                    self.annotationTools.canvas_widget.text_status = Status::TextAdded;
                } else {
                    self.annotationTools.canvas_widget.text_status = Status::TextPressed
                }
                Command::none()
            }
            Message::TextCanvasChanged(string) => {
                self.annotationTools.canvas_widget.textSelected.text = string;
                Command::none()
            }
            Message::SaveTextPosition(cord) => {
                self.annotationTools.canvas_widget.text_status = Status::TextPositioned;
                self.annotationTools.canvas_widget.textSelected.position = cord;
                Command::none()
            }
            Message::CancelColor => {
                //println!("Cancella");
                self.annotationTools.show_color_picker = false;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if window_id == window::Id::MAIN {
            match self.current_page {
                Page::Home => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.home.view())
                }),
                Page::Connection => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.connection.view())
                }),
                Page::ReceiverIp => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.receiver_ip.view())
                }),
                Page::ReceiverStreaming => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.receiver_streaming.view())
                }),
                Page::CasterSettings => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.caster_settings.view())
                }),
                Page::CasterStreaming => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.caster_streaming.view())
                }),
                Page::WindowPartScreen => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.windows_part_screen.view())
                }),
                Page::Shortcut => tokio::task::block_in_place(move || {
                    tokio::runtime::Handle::current().block_on(self.shortcut_screen.view())
                }),
            }
        } else if Some(window_id) == self.second_window_id {
            match self.current_page {
                Page::CasterStreaming => {
                    //println!("devo aggiornare");
                    tokio::task::block_in_place(move || {
                        tokio::runtime::Handle::current().block_on(self.annotationTools.view())
                    })
                }
                _ => {
                    panic!("NON DOVREBBE MAI ENTRARE")
                }
            }
        } else {
            //qui dovrei killare la windows secondaria che non si chiude
            panic!("NON DOVREBBE MAI ENTRARE")
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
            subscriptions.push(
                self.annotationTools
                    .subscription()
                    .map(MessageAnnotation::into),
            )
        }

        Subscription::batch(subscriptions)
    }
}

struct TransparentStyle;

impl container::StyleSheet for TransparentStyle {
    type Style = Style;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let _ = style;
        Appearance {
            text_color: None,
            background: None,
            border: Border {
                color: Color::BLACK,
                width: 2.0,
                radius: Default::default(),
            },
            shadow: Default::default(),
        }
    }
}
