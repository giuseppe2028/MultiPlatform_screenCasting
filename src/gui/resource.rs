#[allow(dead_code)]
pub fn get(file: String) -> String {
    format!("resources/{}", file)
}

pub mod font {
    use iced::font::Font;

    pub const ICON: Font = Font::with_name("home-icon");

    pub const BARLOW:Font = Font::with_name("Barlow-Regular");

    pub const BARLOW_BOLD:Font = Font::with_name("Barlow-Bold");
}
