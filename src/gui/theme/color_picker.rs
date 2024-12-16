use iced::{Background, Color};
use iced_aw::Bootstrap::Border;
use iced_aw::color_picker::{Appearance, StyleSheet};
use crate::gui::theme::Theme;

#[derive(Default)]
pub enum Style {
    #[default]
    Basic,
}

impl StyleSheet for Theme {
    type Style = Self;

    fn active(&self, _: &Self::Style) -> Appearance {

        Appearance{
            background: Background::Color(Color::BLACK),
            border_radius:  15.0,
            border_width: 1.0,
            border_color: Color::WHITE.inverse(),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::WHITE,
        }

    }

    fn selected(&self, _: &Self::Style) -> Appearance {
        Appearance{
            background: Background::Color(Color::BLACK),
            border_radius:  15.0,
            border_width: 1.0,
            border_color: Color::WHITE,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::WHITE,
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance{
            background: Background::Color(Color::BLACK),
            border_radius:  15.0,
            border_width: 1.0,
            border_color: Color::WHITE,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::WHITE,
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        Appearance{
            background: Background::Color(Color::BLACK) ,
            border_radius:  15.0,
            border_width: 1.0,
            border_color: Color::WHITE,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::WHITE,
        }
    }
}