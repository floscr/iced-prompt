use iced::Color;

pub const BUTTON_PRIMARY: Color = Color::from_rgb(
    0x32 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0xC8 as f32 / 255.0,
);

pub const BUTTON_BG_TRANSPARENT: Color = iced::Color {
    r: 255.,
    g: 255.,
    b: 255.,
    a: 0.,
};

pub const BUTTON_BG_SELECTED: Color = iced::Color {
    r: 255.,
    g: 255.,
    b: 255.,
    a: 0.1,
};

pub const BUTTON_SECONDARY: Color = Color::from_rgb(
    0x62 as f32 / 255.0,
    0x69 as f32 / 255.0,
    0x73 as f32 / 255.0,
);
