use iced::{Background, button};
use iced::button::Style;

pub struct TabCloseButton {

}

impl button::StyleSheet for TabCloseButton {
    fn active(&self) -> Style {
        button::Style{
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Default::default()
        }
    }

    fn hovered(&self) -> Style {
        button::Style{
            shadow_offset: Default::default(),
            background: None,
            border_radius: 3.0,
            border_width: 1.0,
            border_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
            text_color: Default::default()
        }
    }

    fn pressed(&self) -> Style {
        Style{
            shadow_offset: Default::default(),
            background: Some(Background::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Default::default()
        }
    }
}
