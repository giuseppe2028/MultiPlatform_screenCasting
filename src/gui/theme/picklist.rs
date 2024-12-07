use iced::Border;
use iced::border::Radius;
use crate::gui::theme::Theme;
use iced::widget::pick_list::{Appearance, StyleSheet};

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &<Self as StyleSheet>::Style) -> Appearance {
        let palette = self.palette();
        Appearance {
            text_color: palette.on_surface,
            placeholder_color: palette.on_surface,
            handle_color: palette.on_surface,
            background: palette.surface.into(),
            border: Border{
                color: palette.outline,
                width: 1.0,
                radius: Radius::from(4.),
            }
        }
    }

    fn hovered(&self, style: &<Self as StyleSheet>::Style) -> Appearance {
        self.active(style)
    }
}
