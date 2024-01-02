use iced::{widget::button, Background, BorderRadius, Color, Vector};

use super::colors;

pub enum ButtonPosition {
    Default,
    Top,
    Bottom,
}

pub enum Button {
    Primary(ButtonPosition),
    Focused(ButtonPosition),
    Secondary,
}

pub const DEFAULT_BORDER_RADIUS: f32 = 6.;

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Button::Primary(_) => colors::BUTTON_BG_TRANSPARENT,
                Button::Focused(_) => colors::BUTTON_BG_SELECTED,
                Button::Secondary => colors::BUTTON_SECONDARY,
            })),
            border_radius: BorderRadius::from(DEFAULT_BORDER_RADIUS),
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active(style)
        }
    }
}

impl From<Button> for iced::theme::Button {
    fn from(style: Button) -> Self {
        iced::theme::Button::Custom(Box::new(style))
    }
}
