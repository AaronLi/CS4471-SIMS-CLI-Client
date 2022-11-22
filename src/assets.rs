use std::io::Cursor;
use image::io::Reader as ImageReader;
use iced::Font;
use iced::pure::widget::{Text};

pub(crate) const SIMS_LOGO_SQUARE: &[u8] = include_bytes!("assets/sims-square.png");
pub(crate) const CLOSE_ICON: &[u8] = include_bytes!("assets/x-circle-fill-light.svg");
pub(crate) const BOOTSTRAP_FONT: Font = Font::External {
    name: "Bootstrap Icons",
    bytes: include_bytes!("assets/bootstrap-icons.ttf")
};

pub(crate) fn get_icon(icon: char) -> Text {
    Text::new(icon).font(BOOTSTRAP_FONT)
}

pub(crate) fn logo_bytes() -> Option<(Vec<u8>, u32, u32)> {
    let image_data = ImageReader::new(Cursor::new(SIMS_LOGO_SQUARE)).with_guessed_format().ok()?.decode().ok()?;
    let image_width = image_data.width();
    let image_height = image_data.height();
    Some((image_data.into_bytes(), image_width, image_height))
}