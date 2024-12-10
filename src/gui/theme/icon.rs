use iced::alignment;
use iced::widget::{Text, text};

/// Material Design Icons
/// https://fonts.google.com/icons
#[derive(Debug, Clone, Copy)]
pub enum Icon {
    StartRecord,
    StopRecord,
    Cancel,
    CasterHome,
    BackUndo,
    BackLeft,
    BackOpen,
    ReceiverHome,
    Play,
    Pause,
    Pencil,
    Rubber,
    Triangle,
    Square,
    Arrow,
    Tools,
    Blanking,
    Phone,
    Text,
    Viewers

}

impl From<&Icon> for char {
    fn from(icon: &Icon) -> Self {
        match icon {
            Icon::CasterHome => '\u{F108}',
            Icon::ReceiverHome => '\u{E800}',
            Icon::BackUndo => '\u{E801}',
            Icon::BackLeft => '\u{E802}',
            Icon::BackOpen => '\u{E803}',
            Icon::StartRecord => '\u{E804}',
            Icon::StopRecord => '\u{F28E}',
            Icon::Cancel => '\u{E805}',
            Icon::Play => '\u{E806}',
            Icon::Pause => '\u{E807}',
            Icon::Pencil => '\u{E808}',
            Icon::Rubber => '\u{F12D}',
            Icon::Triangle => '\u{E809}',
            Icon::Square => '\u{F600}',
            Icon::Arrow => '\u{F178}',
            Icon::Tools => '\u{E80B}',
            Icon::Blanking => '\u{F21B}',
            Icon::Phone => '\u{E80A}',
            Icon::Text => '\u{E80C}',
            Icon::Viewers => '\u{F064}'
        }
    }
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        char::from(&icon)
    }
}

impl ToString for Icon {
    fn to_string(&self) -> String {
        String::from(char::from(self))
    }
}
