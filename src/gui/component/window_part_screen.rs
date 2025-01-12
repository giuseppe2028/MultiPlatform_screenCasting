use crate::gui::app;
use crate::gui::app::Message;
use crate::gui::component::Component;
use crate::gui::theme::button::MyButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::widget::{Column, Element, Row};
use crate::utils::utils::get_screen_dimension;
use iced::mouse;
use iced::widget::{container, row};
use iced::widget::{image, mouse_area, Image};
use iced::{event, Command, Event, Subscription};
use xcap::image::{Rgba, RgbaImage};

pub struct WindowPartScreen {
    pub screenshot: Option<RgbaImage>,
    pub(crate) coordinate: [(f32, f32); 2],
    pub cursor_position: (f32, f32), // Aggiungi un campo per la posizione del mouse
    pub screen_dimension: (f32, f32),
    pub measures: (u32, u32), //vengono usate solo all'inizio per catturare tutto lo schermo prima di selezionare la finestra
    pub draw_rectangle: bool, // Nuovo campo per indicare se disegnare il rettangolo
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
            MessagePress::FirstPress => app::Message::AreaSelectedFirst,
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
                //print!("First press {:?}", self.cursor_position) ;
                self.cursor_position.1 = self.cursor_position.1 - 90.;
                self.coordinate[0] = self.cursor_position;
                self.draw_rectangle = false;
            }
            MessagePress::SecondPress => {
                self.cursor_position.1 = self.cursor_position.1 - 60.;
                self.coordinate[1] = self.cursor_position;
                self.screen_dimension =
                    get_screen_dimension(self.coordinate[0], self.coordinate[1]);
                self.draw_rectangle = true;
            }
            MessagePress::CursorMoved(x, y) => {
                self.cursor_position = (x, y);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, app::Message> {
        let back_button = container(row![
            crate::gui::theme::button::circle_button::CircleButton::new("")
                .style(Style::Danger)
                .icon(crate::gui::theme::icon::Icon::BackLeft)
                .build(20)
                .on_press(app::Message::Back(app::Page::WindowPartScreen))
        ])
        .padding([6, 0, 0, 6])
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top);

        let mut screenshot = self.screenshot.clone().unwrap();

        if self.draw_rectangle {
            let scale_x = screenshot.width() as f32 / 1000.0;
            let scale_y = screenshot.height() as f32 / 625.0;

            // Scala le coordinate
            let start_scaled = (
                (self.coordinate[0].0 * scale_x) as u32,
                (self.coordinate[0].1 * scale_y) as u32,
            );
            let end_scaled = (
                (self.coordinate[1].0 * scale_x) as u32,
                (self.coordinate[1].1 * scale_y) as u32,
            );
            draw_rectangle_on_image(
                &mut screenshot,
                start_scaled,
                end_scaled,
                [255, 0, 0, 255], // Rosso
                3,                // Spessore
            );
        }
        let mouse_area1 = mouse_area(
            Image::new(image::Handle::from_pixels(
                screenshot.width(),
                screenshot.height(),
                screenshot.into_raw(),
            ))
            .width(iced::Length::from(1000))
            .height(iced::Length::from(625)),
        )
        .on_press(MessagePress::FirstPress.into())
        .on_release(MessagePress::SecondPress.into());

        // Costruisci la colonna e restituisci l'elemento
        let col = Column::new()
            .push(Row::new().push(back_button))
            .push(Row::new().push(mouse_area1))
            .push(
                Row::new().push(
                    MyButton::new("CONNECT")
                        .style(Style::Primary)
                        .build()
                        .on_press(Message::StartPartialSharing(
                            self.screen_dimension.0,
                            self.screen_dimension.1,
                            self.coordinate[0].0 as f64,
                            self.coordinate[0].1 as f64,
                        )),
                ),
            );

        col.into()
    }

    // Sottoscrizione agli eventi di movimento del mouse
    // Sottoscrizione agli eventi di movimento del mouse
    fn subscription(&self) -> Subscription<MessagePress> {
        event::listen_with(|event, _status| {
            if let Event::Mouse(mouse::Event::CursorMoved { position }) = event {
                //println!("{} , {}", position.x, position.y);
                Some(MessagePress::CursorMoved(position.x, position.y)) // Send message with new position
            } else {
                None
            }
        })
    }
}

pub fn draw_rectangle_on_image(
    image: &mut RgbaImage,
    start: (u32, u32),
    end: (u32, u32),
    color: [u8; 4], // RGBA color
    thickness: u32,
) {
    let (x1, y1) = start;
    let (x2, y2) = end;

    // Calcola i limiti del rettangolo
    let min_x = x1.min(x2).max(0);
    let max_x = x1.max(x2).min(image.width() - 1);
    let min_y = y1.min(y2).max(0);
    let max_y = y1.max(y2).min(image.height() - 1);

    // Disegna i bordi del rettangolo
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Disegna solo i bordi con lo spessore definito
            let is_border = (x >= min_x && x < min_x + thickness)
                || (x <= max_x && x > max_x - thickness)
                || (y >= min_y && y < min_y + thickness)
                || (y <= max_y && y > max_y - thickness);

            if is_border {
                if let Some(pixel) = image.get_pixel_mut_checked(x, y) {
                    *pixel = Rgba(color);
                }
            }
        }
    }
}

// Aggiungi una funzione helper per un accesso sicuro ai pixel
trait ImageSafeAccess {
    fn get_pixel_mut_checked(&mut self, x: u32, y: u32) -> Option<&mut Rgba<u8>>;
}

impl ImageSafeAccess for RgbaImage {
    fn get_pixel_mut_checked(&mut self, x: u32, y: u32) -> Option<&mut Rgba<u8>> {
        if x < self.width() && y < self.height() {
            Some(self.get_pixel_mut(x, y))
        } else {
            None
        }
    }
}
