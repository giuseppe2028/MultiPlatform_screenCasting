use iced::{Background, Color};
use iced_aw::color_picker::{Appearance, StyleSheet};
use crate::gui::{app::Message, theme::Theme};

use crate::gui::theme::widget::ColorPicker;

#[derive(Default)]
pub enum Style {
    #[default]
    Basic,
}

impl StyleSheet for Theme {
    type Style = Self;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgb8(30, 30, 30)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::WHITE,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::WHITE,
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgb8(40, 40, 40)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(200, 200, 200),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::from_rgb8(200, 200, 200),
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgb8(50, 50, 50)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(255, 255, 255),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::from_rgb8(255, 255, 255),
        }
    }

    fn selected(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgb8(60, 60, 60)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(180, 180, 180),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::from_rgb8(180, 180, 180),
        }
    }
}

pub fn color_picker<'a>(show_picker: bool, color: Color,underlay: super::widget::Element<'a, Message>, on_cancel: Message, on_submit: impl Fn(Color) -> Message + 'static) -> ColorPicker<'a, Message>{
    iced_aw::color_picker(show_picker, color, underlay, on_cancel, on_submit)
}