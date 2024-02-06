pub const ROBOTO_BYTES: &[u8] = include_bytes!("../../fonts/Roboto-Regular.ttf");

pub const ROBOTO: iced::Font = iced::Font {
    weight: iced::font::Weight::Normal,
    family: iced::font::Family::Name("Roboto"),
    monospaced: false,
    stretch: iced::font::Stretch::Normal,
};
