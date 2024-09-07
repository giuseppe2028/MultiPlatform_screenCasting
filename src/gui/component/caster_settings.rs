use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row};
use iced::{Command, Length::Fill};

use crate::column_iced;
use crate::gui::app;
use crate::gui::component::Component;
use crate::gui::theme::button::circle_button::CircleButton;
use crate::gui::theme::button::RectangleButton;
use crate::gui::theme::button::Style;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::widget::Element;

pub struct CasterSettings {}

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
            Message::SelectDisplay(display) => Command::none(),
            Message::GoToConnect => {
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

        let choose_screen_button = RectangleButton::new("Scegli schermo")
            .icon(Icon::CasterHome) // Sostituisci con la tua icona
            .style(Style::Primary)
            .build()
            .on_press(app::Message::from(Message::GoToConnect));

        // Organizzare i pulsanti in una riga o colonna
        container(column_iced![back_button,
               container(
                       row![full_screen_button, window_part_button, choose_screen_button,]
                           .spacing(16) // Spaziatura tra i pulsanti
                           .align_items(iced::Alignment::Center)
               )
               .width(Fill)
               .height(Fill)
               .align_x(Horizontal::Center)
               .align_y(Vertical::Center)
        ])
        .into()
    }
}
