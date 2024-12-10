#[allow(dead_code)]
pub fn get(file: String) -> String {
    format!("resources/{}", file)
}

pub mod font {
    pub const ICON: &[u8]  = include_bytes!("../../resources/home-icon.ttf");

    pub const BARLOW: &[u8] = include_bytes!("../../resources/Barlow-Regular.ttf");

    pub const BARLOW_BOLD: &[u8] = include_bytes!("../../resources/Barlow-Bold.ttf");
}
