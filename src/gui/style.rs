use iced::{
    widget::{button, container, rule, scrollable, svg, text_input},
    Background, BorderRadius, Color,
};

use super::colors;

pub const APP_WINDOW_BORDER_RADIUS: f32 = 10.;
pub const DEFAULT_BORDER_RADIUS: f32 = 5.;

#[derive(Default, Debug, Clone, Copy)]
pub enum ContainerStyle {
    #[default]
    Default,
    Footer,
}

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self {
            ContainerStyle::Footer => container::Appearance {
                text_color: Some(Color {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                    a: 0.35,
                }),
                ..container::Appearance::default()
            },
            _ => container::Appearance {
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
            },
        }
    }
}

pub fn default_container_style() -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(ContainerStyle::Default))
}

pub fn footer_container_style() -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(ContainerStyle::Footer))
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
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::WHITE,
            background: Some(Background::Color(colors::BUTTON_SECONDARY)),
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
            width: 1,
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

#[derive(Default, Debug, Clone, Copy)]
pub enum Svg {
    #[default]
    Default,
}

impl iced::widget::svg::StyleSheet for Svg {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(Color {
                r: 0.6,
                g: 0.6,
                b: 0.6,
                a: 1.,
            }),
        }
    }
}

impl From<Svg> for iced::theme::Svg {
    fn from(style: Svg) -> Self {
        iced::theme::Svg::Custom(Box::new(style))
    }
}

pub fn get_svg_style() -> iced::theme::Svg {
    iced::theme::Svg::Custom(Box::new(Svg::Default))
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Default,
}

impl scrollable::StyleSheet for Scrollable {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: BorderRadius::from(0.),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: Color {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                    a: 0.05,
                },
                border_radius: BorderRadius::from(100.),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        let active = self.active(style);

        let scroller = if is_mouse_over_scrollbar {
            scrollable::Scroller {
                color: Color {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                    a: 0.1,
                },
                ..active.scroller
            }
        } else {
            active.scroller
        };

        scrollable::Scrollbar { scroller, ..active }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.hovered(style, true)
    }
}

impl From<Scrollable> for iced::theme::Scrollable {
    fn from(style: Scrollable) -> Self {
        iced::theme::Scrollable::Custom(Box::new(style))
    }
}
