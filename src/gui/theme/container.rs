use iced::{Background, Border, Color};
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
    Window,
    Container
}

impl StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Style::Container =>
              Appearance{
                  text_color: Some(Color::new(1.0,1.0,1.0,1.0)),
                  background: Some(Background::Color(Color::BLACK)),
                  border: Default::default(),
                  shadow: Default::default(),
              },
            Style::Default => Default::default(),
            Style::OutlinedCard => Appearance {
                background: Some(Background::Color(palette.surface)),
                border:Border{
                    color: palette.outline,
                    width: 1.0,
                    radius:Radius::from(12),
                },
                ..Appearance::default()
            },
            Style::FilledEllipse(fill) => Appearance {
                background: Some(Background::Color(palette.get_palette_color(fill))),
                border:Border{
                    color: Default::default(),
                    width: 0.0,
                    radius: Radius::from(f32::MAX),
                },
                ..Appearance::default()
            },
            Style::Window => Appearance{
                border:Border{
                    color: Color::new(0.8235, 0.1216, 0.1059, 1.0),
                    width: 3.0,
                    radius: Default::default(),
                },
                text_color:Some(Color::BLACK),
                ..Appearance::default()
            }
        }
    }
}
