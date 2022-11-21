use iced::Font;
use iced::widget::{text, Text};

pub(crate) const SIMS_LOGO_SQUARE: &[u8] = include_bytes!("assets/sims-square.png");
pub(crate) const CLOSE_ICON: &[u8] = include_bytes!("assets/x-circle-fill.svg");
pub(crate) const BOOTSTRAP_FONT: Font = Font::External {
    name: "Bootstrap Icons",
    bytes: include_bytes!("assets/bootstrap-icons.ttf")
};

pub(crate) fn get_icon<'a>(icon: char) -> Text<'a> {
    text(icon).font(BOOTSTRAP_FONT)
}