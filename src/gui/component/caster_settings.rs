use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, pick_list, row};
use iced::{Command, Length::Fill, Subscription};
use xcap::Monitor;
use crate::column_iced;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::RectangleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::widget::Element;
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

    fn view(&self) -> Element<'_, app::Message> {
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
            .font(resource::font::BARLOW)
            .width(456);

        // Organizzare i pulsanti in una riga o colonna
        container(column_iced![
            back_button,
            container(
                column_iced![
                    row![full_screen_button, window_part_button]
                        .spacing(16) // Spaziatura tra i pulsanti
                        .align_items(iced::Alignment::Center),
                    row![],
                    row![choose_screen_button].align_items(iced::Alignment::Center)
                ]
                .spacing(15)
            )
            .width(Fill)
            .height(Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
        ])
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        todo!()
    }
}
