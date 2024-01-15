use std::cmp;

use iced::theme::Theme;
use iced::widget::{button, column, container, keyed_column, scrollable, text, text_input};
use iced::window::{self, Level};
use iced::{keyboard, Padding};
use iced::{Application, Element};
use iced::{Command, Length, Settings, Size, Subscription};

use once_cell::sync::Lazy;

mod core;
mod gui;

use gui::style::DEFAULT_BORDER_RADIUS;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    Commands::run(Settings {
        window: window::Settings {
            size: Size::new(700.0, 500.0),
            position: window::Position::Centered,
            transparent: true,
            decorations: false,
            resizable: false,
            level: Level::AlwaysOnTop,
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
enum CommandSelection {
    #[default]
    Initial,
    Selected(i32),
}

#[derive(Debug)]
enum Commands {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    commands: Vec<String>,
    selection: CommandSelection,
}

#[derive(Debug, Clone)]
enum Message {
    IoLoaded(Option<PromptData>),
    InputChanged(String),
    ToggleFullscreen(window::Mode),
    Exit,
    Select(i32),
    Submit,
}

struct ApplicationStyle {}
impl iced::application::StyleSheet for ApplicationStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.6,
            },
            text_color: iced::Color::WHITE,
        }
    }
}

impl Application for Commands {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (Commands, Command<Message>) {
        (
            Commands::Loading,
            Command::perform(PromptData::load(), Message::IoLoaded),
        )
    }

    fn title(&self) -> String {
        "Todos - Iced".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Commands::Loading => {
                #[allow(clippy::single_match)]
                match message {
                    Message::IoLoaded(result) => {
                        *self = match result {
                            Some(data) => Commands::Loaded(State {
                                commands: data.value.split('\n').map(String::from).collect(),
                                ..State::default()
                            }),
                            None => Commands::Loaded(State::default()),
                        };
                    }
                    _ => {}
                }

                text_input::focus(INPUT_ID.clone())
            }
            Commands::Loaded(state) => match message {
                Message::InputChanged(value) => {
                    state.input_value = value;
                    state.selection = CommandSelection::Initial;

                    Command::none()
                }
                Message::Select(amount) => {
                    let selection_index: i32 = match state.selection {
                        CommandSelection::Initial => 0,
                        CommandSelection::Selected(n) => n,
                    };

                    let next_index: i32 = cmp::max(0, selection_index + amount);

                    state.selection = CommandSelection::Selected(next_index);

                    Command::none()
                }
                Message::Submit => {
                    let selection_index: usize = match state.selection {
                        CommandSelection::Initial => 0,
                        CommandSelection::Selected(n) => n as usize,
                    };
                    let filtered_items: Vec<String> =
                        filter_matches(&state.commands, &state.input_value);

                    match filtered_items.get(selection_index) {
                        Some(x) => {
                            println!("{}", x);
                            std::process::exit(0)
                        }
                        _ => std::process::exit(1),
                    }
                }
                Message::Exit => std::process::exit(0),
                Message::ToggleFullscreen(mode) => window::change_mode(window::Id::MAIN, mode),
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        use crate::gui::style::{Button, ButtonPosition, TextInput};

        let default_state = State::default();
        let state = match self {
            Commands::Loading => &default_state,
            Commands::Loaded(state) => state,
        };
        let input_value = &state.input_value;
        let commands = &state.commands;
        let selection = &state.selection;

        let input = text_input("Your prompt", input_value)
            .id(INPUT_ID.clone())
            .style(TextInput::Default)
            .on_submit(Message::Submit)
            .on_input(Message::InputChanged)
            .padding(15)
            .size(30);

        let filtered_items: Vec<String> = filter_matches(commands, input_value);

        let tasks: Option<Element<_>> = if !filtered_items.is_empty() {

            let filtered_items_len = filtered_items.len();

            let elements = keyed_column(filtered_items.iter().enumerate().map(|(i, item)| {
                let button_position = match i {
                    0 => ButtonPosition::Top,
                    _ if i == filtered_items_len - 1 => ButtonPosition::Bottom,
                    _ => ButtonPosition::Default,
                };

                let button_style = match (selection, i) {
                    (CommandSelection::Initial, 0) => Button::Focused(button_position),
                    (CommandSelection::Selected(x), y) if *x == y as i32 => {
                        Button::Focused(button_position)
                    }
                    _ => Button::Primary(button_position),
                };

                (
                    i,
                    button(
                        container(text(item).line_height(3.))
                            .padding(Padding::from([0., DEFAULT_BORDER_RADIUS + 5.])),
                    )
                    .style(button_style)
                    .width(Length::Fill)
                    .into(),
                )
            }))
            .spacing(1)
            .into();

            Some(elements)
        } else {
            None
        };

        let content: Element<_> = match tasks {
            Some(el) => scrollable(container(el).padding(iced::Padding::from([
                10.,
                10. + crate::gui::style::DEFAULT_BORDER_RADIUS,
                10.,
                10.,
            ])))
            .into(),
            _ => container(text("Nothing found"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
        };

        column![input, content].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
            (keyboard::KeyCode::Escape, _) => Some(Message::Exit),
            (keyboard::KeyCode::Up, keyboard::Modifiers::SHIFT) => {
                Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
            }
            (keyboard::KeyCode::Down, keyboard::Modifiers::SHIFT) => {
                Some(Message::ToggleFullscreen(window::Mode::Windowed))
            }
            (keyboard::KeyCode::Up, _) => Some(Message::Select(-1)),
            (keyboard::KeyCode::Down, _) => Some(Message::Select(1)),
            _ => None,
        })
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(Box::new(ApplicationStyle {}))
    }
}

#[derive(Debug, Clone)]
struct PromptData {
    value: String,
}

impl PromptData {
    async fn load() -> Option<PromptData> {
        use async_std::io::ReadExt;

        let mut buffer = String::new();

        let bytes_read = async_std::io::stdin()
            .read_to_string(&mut buffer)
            .await
            .ok()?;

        if bytes_read > 0 {
            Some(PromptData {
                value: buffer.trim().to_string(),
            })
        } else {
            None
        }
    }
}

fn filter_matches(items: &[String], substring: &str) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.to_lowercase().contains(&substring.to_lowercase()))
        .cloned()
        .collect()
}
