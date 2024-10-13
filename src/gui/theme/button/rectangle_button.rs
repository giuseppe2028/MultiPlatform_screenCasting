use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::{button, column, vertical_space};
use iced::{Background, Color};
use std::default::Default;

use crate::gui::theme::button::Style::*;
use crate::gui::theme::button::{Style, Themed};
use crate::gui::theme::color::ColorExt;
use crate::gui::theme::icon::Icon;
use crate::gui::theme::text::{text, bold, icon};
use crate::gui::theme::widget::Button;
use crate::gui::theme::Theme;

pub struct RectangleButton {
    text: String,
    icon: Option<Icon>,
    style: Style,
}

#[allow(dead_code)]
impl RectangleButton {
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
        self.icon = Some(icon);
        self
    }

    pub fn build<'a, Message: 'a>(self) -> Button<'a, Message> {
        if let Some(_icon) = self.icon.clone() {
            button(
                column![
                    vertical_space(12),
                    icon(_icon).size(48),  // Icona al centro e grande
                    vertical_space(14),
                    text(self.text.clone()).size(20)  // Testo sotto
                ]
                .align_items(iced::Alignment::Center)  // Allinea il contenuto al centro
            )
        } else {
            button(column![
                bold(self.text.clone()).size(20)
            ]
            .align_items(iced::Alignment::Center))
        }
        .style(Box::new(self) as _)
        .padding([10, 20])  // Aggiungi padding per migliorare l'aspetto
        .height(130)
        .width(200)
    }
}

impl Themed for RectangleButton {}

impl StyleSheet for RectangleButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = style.palette();
        let partial = Appearance {
            border_radius: 12.0,
            ..Appearance::default()
        };
        let from = |background: Color, on_background: Color| Appearance {
            background: background.into(),
            text_color: on_background,
            ..partial
        };

        match self.style {
            Default => from(palette.surface, palette.on_surface),
            Primary => from(palette.primary, palette.on_primary),
            Secondary => from(palette.secondary, palette.on_secondary),
            Danger => from(palette.error, palette.on_error),
            Success => from(palette.success, palette.on_success),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let palette = style.palette();
        let base = self.active(style);
        let state = match self.style {
            Default => palette.on_surface,
            Primary => palette.on_primary,
            Secondary => palette.on_secondary,
            Danger => palette.on_error,
            Success => palette.on_success,
        };

        Appearance {
            background: base.background.map(|background| match background {
                Background::Color(color) => Background::Color(color.mix(state.with_alpha(0.12))),
            }),
            ..base
        }
    }
}