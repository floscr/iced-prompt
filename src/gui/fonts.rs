pub const ROBOTO: &[u8] = include_bytes!("../../fonts/Roboto-Regular.ttf");

pub const roboto: iced::Font = iced::Font {
    weight: iced::font::Weight::Normal,
    family: iced::font::Family::Name("Roboto"),
    monospaced: false,
    stretch: iced::font::Stretch::Normal,
};
