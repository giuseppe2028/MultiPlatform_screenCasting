use std::f32::MAX;
use iced::Background::Color;
use iced::Border;
use iced::border::Radius;
use iced::theme::palette::Background;
use crate::gui::theme::color::ColorExt;
use crate::gui::theme::Theme;
use iced::widget::scrollable::{Appearance, Scrollbar, Scroller, StyleSheet};
use iced_aw::Icon::App;
use crate::gui::app::Message::Back;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        let palette = self.palette();
        Appearance {
            container: Default::default(),
            scrollbar:
            Scrollbar {
            //TODO refactor
            background: None,
            border: Border{
                color: palette.outline,
                width: 0.0,
                radius: Radius::from(f32::MAX),
            },
            scroller: Scroller {
                color: palette.on_surface.with_alpha(0.1),
                border: Border{
                    color: palette.outline,
                    width: 0.0,
                    radius:  Radius::from(f32::MAX),
                },
            },
        },
            gap: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Appearance {
        let palette = self.palette();
        Appearance{
            container: Default::default(),
            scrollbar: Scrollbar {
                background: Some(Color(palette.surface)),
                border: Border{
                    color: palette.outline,
                    width: 0.0,
                    radius: Radius::from(f32::MAX),
                },
                scroller: Scroller {
                    color: palette.on_surface,
                    border: Border{
                        color: palette.outline,
                        width: 0.0,
                        radius: Radius::from([30.0;4]),
                    },
                },
            },
            gap: None,
        }
    }
}