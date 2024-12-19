use iced::{Background, Border};
use iced::border::Radius;
use crate::gui::theme::color::ColorExt;
use crate::gui::theme::Theme;
use iced::widget::scrollable::{Scrollbar, Scroller, StyleSheet};

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
        let palette = self.palette();
        iced::widget::scrollable::Appearance{
            container: Default::default(),
            scrollbar: Scrollbar {
                background: Some(Background::Color(palette.surface)),
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
                        radius: Radius::from(f32::MAX),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_mouse_over_scrollbar: bool) -> iced::widget::scrollable::Appearance {
        let palette = self.palette();
        iced::widget::scrollable::Appearance{
            container: Default::default(),
            scrollbar: Scrollbar {
                background: Some(Background::Color(palette.surface)),
                scroller: Scroller {
                    color: palette.on_surface,
                    border: Border{
                        color: palette.outline,
                        width: 0.0,
                        radius: Radius::from(f32::MAX),
                    },
                },
                border: Border{
                    color: palette.outline,
                    width: 0.0,
                    radius: Radius::from(f32::MAX),
                },
            },
            gap: None,
        }

    }
}
