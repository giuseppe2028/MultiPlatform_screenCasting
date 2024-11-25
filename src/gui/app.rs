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
use crate::gui::component::window_part_screen::{MessagePress, WindowPartScreen};
use crate::gui::component::{home, Component};
use crate::gui::theme::widget::Element;
use crate::gui::theme::Theme;
use crate::socket::socket::{CasterSocket, ReceiverSocket};
use iced::time::{self, Duration};
use iced::{executor, Application, Command, Subscription};
use std::sync::Arc;
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
}

enum Controller {
    ReceiverController(ReceiverController),
    CasterController(AppController),
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
    Route(Page),
    StartSharing,
    RoleChosen(home::Message),
    ReceiverSharing(String),
    ReceiverInputIp(receiver_ip::Message),
    SetSettingsCaster(caster_settings::Window),
    Back(Page),
    StartRecording(receiver_streaming::UpdateMessage),
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
    CasterControllerCreated(CasterSocket, Sender<RgbaImage>, Page),
    ReceiverControllerCreated(ReceiverSocket, Sender<RgbaImage>, Page),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (sender_caster, receiver_caster) = channel::<RgbaImage>(32); // Define buffer size
        let (sender_receiver, receiver_receiver) = channel::<RgbaImage>(32); // Define buffer size

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
                        let sender = self.sender_caster.clone();
                        Command::perform(
                            async move {
                                let socket =
                                    crate::socket::socket::CasterSocket::new("127.0.0.1:8000")
                                        .await;
                                /*let caster_controller = Controller::CasterController(AppController::new(
                                    Monitor::all().unwrap().get(0).unwrap().clone(),
                                    sender,
                                    socket,
                                ));*/
                                let page = Page::CasterSettings;
                                (socket, sender, page)
                            },
                            |(socket, sender, page)| {
                                // Once the operation is complete, send a "ControllerCreated" message
                                //self.controller = caster_controller;
                                Message::CasterControllerCreated(socket, sender, page)
                            },
                        )
                    }
                    Role::Receiver => {
                        self.current_page = Page::Selection;
                        Command::none()
                    }
                },
            },
            Message::Route(page) => Command::none(),
            Message::CasterControllerCreated(socket, sender, page) => {
                self.controller = Controller::CasterController(AppController::new(
                    Monitor::all().unwrap().get(0).unwrap().clone(),
                    sender,
                    socket,
                ));
                self.current_page = page;
                Command::none()
            }
            Message::StartSharing => {
                self.current_page = Page::CasterStreaming;
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster.listens_for_receivers();
                    caster.start_sharing();
                    self.caster_streaming.measures = caster.get_measures();
                }
                Command::none()
            }
            Message::ReceiverSharing(ip_caster) => {
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
                    |(socket, sender, page)| {
                        // Once the operation is complete, send a "ControllerCreated" message
                        //self.controller = caster_controller;
                        Message::ReceiverControllerCreated(socket, sender, page)
                    },
                )
            },
            Message::ReceiverControllerCreated(socket, sender , page ) => {
                self.controller = Controller::ReceiverController(ReceiverController::new(
                    sender,
                    socket,
                ));
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
                            async {
                                self.windows_part_screen.screenshot =
                                    Some(caster.take_screenshot().await);
                            };
                            self.windows_part_screen.measures = caster.get_measures();
                        }
                        self.current_page = Page::WindowPartScreen
                    }
                }
                Command::none()
            }
            Message::StartRecording(message) => {
                let _ = self.receiver_streaming.update(message);
                Command::none()
            }
            Message::TogglerChanged(message) => {
                let _ = self.caster_streaming.update(message);
                Command::none()
            }
            Message::SelectDisplay(display) => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster.set_display(display.clone());
                }
                let _ = self
                    .caster_settings
                    .update(caster_settings::Message::SelectDisplay(display));
                Command::none()
            }
            Message::Close => {
                if let Controller::CasterController(caster) = &mut self.controller {
                    caster.stop_streaming();
                } else if let Controller::ReceiverController(receiver) = &mut self.controller {
                    receiver.stop_streaming();
                }
                self.current_page = Page::Home;
                Command::none()
            }
            Message::UpdateScreen => {
                match &self.controller {
                    Controller::ReceiverController(_) => {
                        let frame = {
                            if let Ok(receiver) =
                                self.receiver_streaming.receiver.blocking_lock().try_recv()
                            {
                                receiver
                            } else {
                                return Command::none();
                            }
                        };
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
            },
            
            _ => Command::none(),
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
