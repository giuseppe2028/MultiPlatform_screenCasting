use enigo::{Enigo, Mouse, Settings};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Image, image, mouse_area, row, text};
use iced::{Application, Command, Event, event, Subscription};
use iced::keyboard::Event::KeyPressed;
use iced::Length::{Fill, Shrink};
use iced::mouse::Event::CursorMoved;
use iced::widget::mouse_area::MouseArea;
use iced_aw::Icon::{Cursor};

use iced::mouse;
use crate::column_iced;
use crate::gui::app;
use crate::gui::app::Message;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::bold;
use crate::gui::theme::widget::{Container, Element};
use crate::utils::utils::{calculate_distance, calculate_screen_percentage, get_scale_factor, get_screen_dimension};
// use crate::gui::theme::widget::TextInput;

pub struct WindowPartScreen {
    pub screenshot: Vec<u8>,
    pub(crate) coordinate: [(f32, f32); 2],
    pub cursor_position: (f32, f32), // Aggiungi un campo per la posizione del mouse
    pub screen_dimension:(f32,f32)
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
                println!("Prima premuto");
                self.coordinate[0] = self.cursor_position;
                println!("Press at x,y: {:?}", self.cursor_position);
            }
            MessagePress::SecondPress=>{
                self.coordinate[1] = self.cursor_position;
                println!("Seconda premuto");
                println!("Rilasciato at x,y: {:?}", self.cursor_position);
                self.screen_dimension = get_screen_dimension(self.coordinate[0],self.coordinate[1]);
            }
            MessagePress::CursorMoved(x, y) => {
                self.cursor_position = (x, y);
                println!("Mouse moved to x: {} y: {}", x, y); // Stampa della posizione del mouse
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let mouse_area1 = mouse_area(
            Image::new(image::Handle::from_pixels(1440, 900, rgb_to_rgba(self.screenshot.clone())))
                .width(iced::Length::from(1000))
                .height(iced::Length::from(625))
        )
            .on_press(MessagePress::FirstPress.into())
            .on_release(MessagePress::SecondPress.into());

        // Costruisci la colonna e restituisci l'elemento
        column_iced![row![mouse_area1],row![
            MyButton::new("CONNECT")
                    .style(Style::Primary)
                    .build()
            //TODO Modificare
                    .on_press(Message::StartPartialSharing(self.screen_dimension.0,self.screen_dimension.1,self.coordinate[0].0 as f64,self.coordinate[0].1 as f64).into())
        ]
                .align_items(iced::Alignment::Center)
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

fn rgb_to_rgba(rgb_buffer: Vec<u8>) -> Vec<u8> {
    let rgb_len = rgb_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((rgb_len / 3) * 4); // Ogni pixel RGB diventa RGBA

    // Itera i pixel RGB e aggiungi il canale Alpha
    for rgb_chunk in rgb_buffer.chunks_exact(3) {
        rgba_buffer.push(rgb_chunk[0]); // Red
        rgba_buffer.push(rgb_chunk[1]); // Green
        rgba_buffer.push(rgb_chunk[2]); // Blue
        rgba_buffer.push(255);          // Alpha (opaco)
    }

    rgba_buffer
}
