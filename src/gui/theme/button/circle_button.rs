use iced::widget::button;
use iced::widget::button::{Appearance, StyleSheet};
use iced::{Background, Border, Color};
use std::default::Default;
use iced::border::Radius;
use crate::gui::theme::button::{Style, Themed};
use crate::gui::theme::color::ColorExt;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::{bold, icon};
use crate::gui::theme::widget::Button;
use crate::gui::theme::Theme;

pub struct CircleButton {
    text: String,
    icon: Option<Icon>,
    style: Style,
}

#[allow(dead_code)]
impl CircleButton {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            icon: None,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn build<'a, Message: 'a>(self, size: u16) -> Button<'a, Message> {
        let content = if let Some(_icon) = self.icon.clone() {
            // Centrare solo l'icona
            icon(_icon).size(size) // Regola la dimensione dell'icona se necessario
        } else {
            // Se non c'è icona, usa il testo (anche se per un cerchio ci si aspetta solo un'icona)
            bold(self.text.clone()).size(size)
        }
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center);

        Button::new(content)
            .style(Box::new(self) as Box<dyn Themed>)
            .padding(0) // Rimuove padding aggiuntivo per centrare meglio l'icona
            .width(60)
            .height(60)

    }
}

impl Themed for CircleButton {}

impl StyleSheet for CircleButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = style.palette();
        let appearance = match self.style {
            Style::Primary => palette.button_color_primary,
            Style::Danger => palette.button_color_danger,
            _ => palette.button_color_default,
        };

        Appearance {
            background: Some(Background::Color(appearance)),
            border:Border{
                color: Default::default(),
                width: 0.0,
                radius: Radius::from(32.),
            },
            text_color: Color::WHITE,
            ..Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let palette = style.palette();
        let active = self.active(style);

        let hover_color = match self.style {
            Style::Primary => palette.button_color_primary, // Usa lo stesso colore per il fondo
            Style::Danger => palette.button_color_danger,   // Usa lo stesso colore per il fondo
            _ => palette.button_color_default,              // Usa lo stesso colore per il fondo
        };

        // Schiarisci leggermente il colore del testo quando il pulsante è in stato di hover
        let hover_text_color = active.text_color.mix(Color::WHITE.with_alpha(0.2));

        Appearance {
            background: Some(Background::Color(
                hover_color.mix(Color::WHITE.with_alpha(0.1)),
            )),
            border:Border{
                color: Default::default(),
                width: 0.0,
                radius: active.border.radius,
            },
            text_color: hover_text_color,
            ..active
        }
    }
}
