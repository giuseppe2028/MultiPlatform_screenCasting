use iced::Border;
use iced::border::Radius;
use crate::gui::theme::Theme;
use iced::overlay::menu::{Appearance, StyleSheet};

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.palette().on_surface,
            background: self.palette().surface.into(),
            border:Border{
                color:  self.palette().outline,
                width: 1.0,
                radius: Radius::from(4.),
            },
            selected_text_color: self.palette().on_primary,
            selected_background: self.palette().primary.into(),
        }
    }
}
