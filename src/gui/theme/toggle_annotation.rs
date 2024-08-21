use iced::widget::toggler::{Appearance, StyleSheet};
use crate::gui::theme::widget::Toggler;
use iced::Color;

use crate::gui::app::Message;
use crate::gui::resource;
use crate::gui::theme::Theme;

#[allow(dead_code)]
#[derive(Default)]
pub enum Style {
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = Style;

    fn active(&self, style: &Self::Style, is_active: bool) -> Appearance {
        Appearance {
            background: Color::BLACK,
            background_border: Some(Color::from_rgb(0.7, 0.7, 0.7)),
            foreground: Color::WHITE,
            foreground_border: Some(Color::from_rgb(0.7, 0.7, 0.7)),
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        Appearance {
            background: Color::WHITE,
            background_border: Some(Color::from_rgb(0.7, 0.7, 0.7)),
            foreground: Color::BLACK,
            foreground_border: Some(Color::from_rgb(0.7, 0.7, 0.7)),
        }
    }
}

pub fn toggler<'a>(text:  impl Into<Option<String>>, is_checked: bool, callback: impl Fn(bool) -> Message + 'a ) -> Toggler<'a> {
    iced::widget::toggler(text, is_checked, callback)
        .font(resource::font::BARLOW)
}