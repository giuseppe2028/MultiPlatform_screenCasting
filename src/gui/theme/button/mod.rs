use iced::widget::button::{Appearance, StyleSheet};

pub use button::MyButton;
pub use rectangle_button::RectangleButton;

use crate::gui::theme::Theme;

mod button;
pub mod circle_button;
mod rectangle_button;

#[allow(dead_code)]
#[derive(Default)]
pub enum Style {
    #[default]
    Default,
    Primary,
    Secondary,
    Danger,
    Success,
}

pub trait Themed: StyleSheet<Style = Theme> {}

impl StyleSheet for Theme {
    type Style = Box<dyn Themed<Style = Theme>>;

    fn active(&self, style: &Self::Style) -> Appearance {
        style.active(self)
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        style.hovered(self)
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        style.pressed(self)
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        style.disabled(self)
    }
}

struct DefaultButton;

impl Themed for DefaultButton {}

impl StyleSheet for DefaultButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance::default()
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance::default()
    }

    fn pressed(&self, _: &Self::Style) -> Appearance {
        Appearance::default()
    }

    fn disabled(&self, _: &Self::Style) -> Appearance {
        Appearance::default()
    }
}

impl Default for Box<dyn Themed<Style = Theme>> {
    fn default() -> Self {
        Box::new(DefaultButton)
    }
}
