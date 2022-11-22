use iced::{Theme, theme};
use iced::widget::{button};
use iced::widget::button::Appearance;
use num_traits::float::Float;

#[derive(Default)]
pub struct Fab; // Floating Action Button

impl button::StyleSheet for Fab {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance{
            border_radius: f32::infinity(),
            ..style.active(&theme::Button::Primary)
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        Appearance{
            border_radius: f32::infinity(),
            ..style.hovered(&theme::Button::Primary)
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance{
            border_radius: f32::infinity(),
            ..style.pressed(&theme::Button::Primary)
        }
    }
}