use iced::Background::Color;
use iced::Border;
use iced::border::Radius;
use iced::widget::container::{Appearance, StyleSheet};

use crate::gui::theme::{PaletteColor, Theme};

#[allow(dead_code)]
#[derive(Default)]
pub enum Style {
    /// Material Design 3 Outlined Card
    /// https://m3.material.io/components/cards/specs#9ad208b3-3d37-475c-a0eb-68cf845718f8
    #[default]
    Default,
    OutlinedCard,
    FilledEllipse(PaletteColor),
}

impl StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Style::Default => Default::default(),
            Style::OutlinedCard => Appearance {
                background: Some(Color(palette.surface)),
                border:Border{
                    color: palette.outline,
                    width: 1.0,
                    radius: Radius::from(12),
                },
                ..Appearance::default()
            },
            Style::FilledEllipse(fill) => Appearance {
                background: Some(Color(palette.get_palette_color(fill))),
                border:Border{
                    color: Default::default(),
                    width: 0.0,
                    radius: Radius::from(f32::MAX),
                },
                ..Appearance::default()
            },
        }
    }
}
