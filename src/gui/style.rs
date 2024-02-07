use iced::{
    widget::{button, container, rule, text_input},
    Background, BorderRadius, Color, Vector,
};

use super::colors;

pub const APP_WINDOW_BORDER_RADIUS: f32 = 10.;
pub const DEFAULT_BORDER_RADIUS: f32 = 5.;

#[derive(Default, Debug, Clone, Copy)]
pub enum ContainerStyle {
    #[default]
    Default,
}

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.55,
            })),
            border_radius: BorderRadius::from(APP_WINDOW_BORDER_RADIUS),
            border_width: 0.5,
            border_color: Color {
                r: 0.25,
                g: 0.25,
                b: 0.25,
                a: 0.1,
            },
            ..container::Appearance::default()
        }
    }
}

pub fn get_item_container_style() -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(ContainerStyle::Default))
}

pub enum Button {
    Primary,
    Focused,
    Secondary,
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Button::Primary => colors::BUTTON_BG_TRANSPARENT,
                Button::Focused => colors::BUTTON_BG_SELECTED,
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

#[derive(Default, Debug, Clone, Copy)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for TextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(colors::BUTTON_BG_TRANSPARENT),
            border_width: 0.0,
            border_radius: 0.0.into(),
            border_color: Color::TRANSPARENT,
            icon_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(colors::BUTTON_BG_TRANSPARENT),
            border_width: 0.0,
            ..self.active(style)
        }
    }

    // Implement the missing trait items with default values
    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Default::default()
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Default::default()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}

impl From<TextInput> for iced::theme::TextInput {
    fn from(style: TextInput) -> Self {
        iced::theme::TextInput::Custom(Box::new(style))
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for Rule {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: Color {
                r: 1.,
                g: 1.,
                b: 1.,
                a: 0.01,
            },
            width: 0,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}

impl From<Rule> for iced::theme::Rule {
    fn from(style: Rule) -> Self {
        iced::theme::Rule::Custom(Box::new(style))
    }
}
