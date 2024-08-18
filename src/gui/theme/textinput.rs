use iced::widget::text_input::{Appearance, StyleSheet};

use crate::gui::resource;
use crate::gui::theme::{PaletteColor, Theme};
use crate::gui::theme::widget::TextInput;

#[allow(dead_code)]
#[derive(Default)]
pub enum Style {
    /// Material Design 3 Outlined Card
    /// https://m3.material.io/components/cards/specs#9ad208b3-3d37-475c-a0eb-68cf845718f8
    #[default]
    Default,

}

impl StyleSheet for Theme {
    type Style = Style;
    
    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            background: todo!(),
            border_radius: todo!(),
            border_width: todo!(),
            border_color: todo!(),
            icon_color: todo!(),
        }
    }
    
    fn focused(&self, style: &Self::Style) -> Appearance {
        todo!()
    }
    
    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }
    
    fn value_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }
    
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }
    
    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }
    
    fn disabled(&self, style: &Self::Style) -> Appearance {
        todo!()
    }
    
}

pub fn textinput<'a>(placeholder: &str, value: &str) -> TextInput<'a> {
    iced::widget::text_input(placeholder, value).font(resource::font::BARLOW)
}