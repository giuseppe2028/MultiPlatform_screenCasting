use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{Background, Color};

use crate::gui::resource;
use crate::gui::theme::widget::TextInput;
use crate::gui::theme::Theme;

#[allow(dead_code)]
#[derive(Default)]
pub enum Style {
    /// Material Design 3 Outlined Card
    /// https://m3.material.io/components/cards/specs#9ad208b3-3d37-475c-a0eb-68cf845718f8
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = Style;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::WHITE),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            icon_color: Color::from_rgb(0.3, 0.3, 0.3),
        }
    }

    fn focused(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::WHITE),
            border_radius: 5.0,
            border_width: 2.0,
            border_color: Color::from_rgb(0.2, 0.6, 0.8),
            icon_color: Color::from_rgb(0.2, 0.6, 0.8),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        Color::BLACK
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.2, 0.6, 0.8)
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            icon_color: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }
}
pub fn textinput<'a>(placeholder: &str, value: &str) -> TextInput<'a> {
    iced::widget::text_input(placeholder, value).font(resource::font::BARLOW)
}
