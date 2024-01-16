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

use core::cmd;
use gui::style::DEFAULT_BORDER_RADIUS;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
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
    cmds: Vec<cmd::Cmd>,
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
            background_color: iced::Color::TRANSPARENT,
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
        "Iced Query".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Commands::Loading => {
                #[allow(clippy::single_match)]
                match message {
                    Message::IoLoaded(result) => {
                        *self = match result {
                            Some(data) => Commands::Loaded(State {
                                cmds: data.value.split('\n').map(|x| cmd::Cmd::new(x.to_string())).collect(),
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
                    let filtered_items: Vec<cmd::Cmd> =
                        filter_matches(&state.cmds, &state.input_value);

                    match filtered_items.get(selection_index) {
                        Some(cmd) => {
                            let value = cmd::Cmd::value(cmd);
                            println!("{}", value);
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
        use crate::gui::style::{get_item_container_style, Button, ButtonPosition, TextInput};

        let default_state = State::default();
        let state = match self {
            Commands::Loading => &default_state,
            Commands::Loaded(state) => state,
        };
        let input_value = &state.input_value;
        let cmds = &state.cmds;
        let selection = &state.selection;

        let input = text_input("Your prompt", input_value)
            .id(INPUT_ID.clone())
            .style(TextInput::Default)
            .on_submit(Message::Submit)
            .on_input(Message::InputChanged)
            .padding(Padding::from([15., DEFAULT_BORDER_RADIUS + 10.]))
            .size(15.);

        let filtered_cmds: Vec<cmd::Cmd> = filter_matches(cmds, input_value);

        let cmds: Option<Element<_>> = if !filtered_cmds.is_empty() {
            let filtered_items_len = filtered_cmds.len();


            let elements = keyed_column(
                filtered_cmds.iter().enumerate().map(|(i, cmd)| {
                    let value = cmd::Cmd::value(cmd);
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
                        container(text(value).line_height(1.25))
                            .padding(Padding::from([5., DEFAULT_BORDER_RADIUS + 5.])),
                    )
                    .style(button_style)
                    .width(Length::Fill)
                    .into(),
                )
            }
                ))
            .spacing(1)
            .into();

            Some(elements)
        } else {
            None
        };

        let content: Element<_> = match cmds {
            Some(el) => scrollable(container(el).padding(iced::Padding::from([
                5.,
                10. + crate::gui::style::DEFAULT_BORDER_RADIUS,
                10.,
                10.,
            ])))
                .id(SCROLLABLE_ID.clone())
                .into(),
            _ => container(text("Nothing found"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
        };

        container(column![input, content])
            .height(Length::Fill)
            .style(get_item_container_style())
            .into()
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

fn filter_matches(items: &[cmd::Cmd], substring: &str) -> Vec<cmd::Cmd> {
    items
        .iter()
        .filter_map(|cmd| {
                let value = cmd::Cmd::value(cmd);
                let is_match = value.to_lowercase().contains(&substring.to_lowercase());

                if is_match { Some(cmd) } else { None }
        })
        .cloned()
        .collect()
}
