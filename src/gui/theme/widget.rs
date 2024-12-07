#![allow(dead_code)]

use iced::widget::image::Handle;
use crate::gui::app::Message;


pub type Renderer = iced::Renderer<crate::gui::theme::Theme>;
pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Renderer>;
pub type Tooltip<'a> = iced::widget::Tooltip<'a, Renderer>;
pub type ProgressBar = iced::widget::ProgressBar<Renderer>;
pub type PickList<'a, T, Message> = iced::widget::PickList<'a, T, Message, Renderer, Message>;
pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
pub type Svg = iced::widget::Svg<Renderer>;

pub type Image = iced::widget::Image<Handle>;

pub type TextInput<'a> = iced::widget::text_input::TextInput<'a, Message, Renderer>;
pub type Toggler<'a> = iced::widget::toggler::Toggler<'a, Message, Renderer>;

pub type Tabs<'a, Message> = iced_aw::native::Tabs<'a, Message, Renderer, Renderer>;
