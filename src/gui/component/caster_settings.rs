use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, horizontal_space, pick_list, row, vertical_space};
use iced::{Command, Length::Fill};

use crate::column_iced;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::RectangleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::widget::Element;
use crate::gui::{app, resource};

pub struct CasterSettings {
    pub available_displays: Vec<String>,
    pub selected_display: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectDisplay(String), // Cambiare tipo nel display corrispondente
    SelectWindow,          // Probabilmente avrà bisogno di parametri
    GoToConnect,
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        app::Message::SetSettingsCaster(message)
    }
}

impl<'a> Component<'a> for CasterSettings {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced::Command<app::Message> {
        match message {
            Message::SelectDisplay(display) => {
                self.selected_display = Some(display);
                Command::none()
            }
            Message::GoToConnect => {
                //quando conferma tutto ciò che ha scelto (se non sceglie nulla di defualt full screen schermo principale)
                // Funzioni di backend
                Command::none()
            }
            Message::SelectWindow => todo!(),
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
            .on_press(app::Message::from(Message::SelectDisplay(
                "Schermo intero".to_string(),
            )));

        let window_part_button = RectangleButton::new("Porzione di finestra")
            .icon(Icon::CasterHome) // Sostituisci con la tua icona
            .style(Style::Primary)
            .build()
            .on_press(app::Message::from(Message::SelectWindow));

        let choose_screen_button = pick_list(
            self.available_displays.clone(),
            self.selected_display.clone(),
            move |message| app::Message::SelectDisplay(Message::SelectDisplay(message)),
        )
        .placeholder("Select the screen")
        .font(resource::font::BARLOW)
        .width(416);

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
}