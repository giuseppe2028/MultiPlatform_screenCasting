use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, pick_list, row, Row};
use iced::{Command, Length::Fill, Subscription};
use xcap::Monitor;

use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::RectangleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::widget::{Column, Container, Element};
use crate::gui::{app, resource};

pub struct CasterSettings {
    pub available_displays: Vec<Monitor>,
    pub selected_display: Monitor
}

#[derive(Debug, Clone)]
pub enum Window {
    FullScreen,
    Area,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectDisplay(Monitor), // Cambiare tipo nel display corrispondente
    SelectWindow(Window),                  // Probabilmente avrà bisogno di parametri
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        match message {
            Message::SelectDisplay(display) => {
                return app::Message::SelectDisplay(display);
            }
            Message::SelectWindow(window) => {
                return app::Message::SetSettingsCaster(window);
            }
        }
    }
}

impl<'a> Component<'a> for CasterSettings {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::SelectDisplay(display) => {
                self.selected_display = display;
                Command::none()
            }
            Message::SelectWindow(_window) => todo!(),
        }
    }

    fn view(&self /*, props: Self::ViewProps*/) -> Element<'_, app::Message> {
        let back_button = container(row![CircleButton::new("")
            .style(Style::Danger)
            .icon(Icon::BackLeft)
            .build(20)
            .on_press(app::Message::Back(app::Page::CasterSettings))])
        .padding([6, 0, 0, 6])
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top);

        let full_screen_button = RectangleButton::new("Schermo intero")
            .icon(Icon::CasterHome) // Sostituisci con la tua icona
            .style(Style::Primary)
            .build()
            .on_press(app::Message::from(Message::SelectWindow(
                Window::FullScreen,
            )));

        let window_part_button = RectangleButton::new("Porzione di finestra")
            .icon(Icon::CasterHome) // Sostituisci con la tua icona
            .style(Style::Primary)
            .build()
            .on_press(app::Message::from(Message::SelectWindow(Window::Area))); //TODO TOIMPLEMENT

        let choose_screen_button = pick_list(
            self.available_displays.clone(),
            Some(self.selected_display.clone()),
            move |message| app::Message::SelectDisplay(message),
        )

        .width(416);

        // Organizzare i pulsanti in una riga o colonna
        container(
            Column::new()
                .push(back_button)
                .push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(full_screen_button)
                                    .push(window_part_button)
                                    .spacing(16)
                                    .align_items(iced::Alignment::Center)
                            )
                            .push(Row::new().push(choose_screen_button)).align_items(iced::Alignment::Center)
                            .spacing(16)
                    )   .width(Fill)
                        .height(Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                )
                ).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
