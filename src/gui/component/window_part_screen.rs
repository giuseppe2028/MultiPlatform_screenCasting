use iced::widget::{Image, image, mouse_area, row};
use iced::{Command, Event, Subscription};

use iced::mouse;
use xcap::image::RgbaImage;
use crate::gui::app;
use crate::gui::app::Message;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::widget::Element;
use crate::utils::utils:: get_screen_dimension;

pub struct WindowPartScreen {
    pub screenshot: Option<RgbaImage>,
    pub(crate) coordinate: [(f32, f32); 2],
    pub cursor_position: (f32, f32), // Aggiungi un campo per la posizione del mouse
    pub screen_dimension:(f32,f32),
    pub measures: (u32, u32) //vengono usate solo all'inizio per catturare tutto lo schermo prima di selezionare la finestra
}

#[derive(Debug, Clone)]
pub enum MessagePress {
    FirstPress,
    SecondPress,
    CursorMoved(f32, f32), // Nuovo messaggio per la posizione del mouse
}

impl From<MessagePress> for app::Message {
    fn from(message: MessagePress) -> Self {
        match message {
            MessagePress::FirstPress=> app::Message::AreaSelectedFirst,
            MessagePress::SecondPress => app::Message::AreaSelectedSecond,
            MessagePress::CursorMoved(x, y) => app::Message::CursorMoved(x, y), // Messaggio per movimento del mouse
        }
    }
}

impl<'a> Component<'a> for WindowPartScreen {
    type Message = MessagePress;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            MessagePress::FirstPress => {
                print!("First press {:?}", self.cursor_position) ;
                self.coordinate[0] = self.cursor_position;
            }
            MessagePress::SecondPress=>{
                self.coordinate[1] = self.cursor_position;
                self.screen_dimension = get_screen_dimension(self.coordinate[0],self.coordinate[1]);
            }
            MessagePress::CursorMoved(x, y) => {
                self.cursor_position = (x, y);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let screenshot = self.screenshot.clone().unwrap();
        let mouse_area1 = mouse_area(
            Image::new(image::Handle::from_pixels(screenshot.width(), screenshot.height(), screenshot.into_raw()))
                .width(iced::Length::from(1000))
                .height(iced::Length::from(625))
        )
            .on_press(MessagePress::FirstPress.into())
            .on_release(MessagePress::SecondPress.into());

        // Costruisci la colonna e restituisci l'elemento
        column![row![mouse_area1],row![
            MyButton::new("CONNECT")
                    .style(Style::Primary)
                    .build()
                    .on_press(Message::StartPartialSharing(self.screen_dimension.0,self.screen_dimension.1,self.coordinate[0].0 as f64,self.coordinate[0].1 as f64))
        ]

        ].into()
    }


    // Sottoscrizione agli eventi di movimento del mouse
    // Sottoscrizione agli eventi di movimento del mouse
    fn subscription(&self) -> Subscription<MessagePress> {
        iced::subscription::events_with(|event, _status| {
            if let Event::Mouse(mouse::Event::CursorMoved { position }) = event {
                Some(MessagePress::CursorMoved(position.x, position.y)) // Send message with new position
            } else {
                None
            }
        })
    }

}