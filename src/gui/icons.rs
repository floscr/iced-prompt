use iced::advanced::svg;
use once_cell::sync::Lazy;

pub static DIRECTORY: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(include_bytes!("../../icons/directory.svg").to_vec()));

pub static FILE: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(include_bytes!("../../icons/file.svg").to_vec()));
