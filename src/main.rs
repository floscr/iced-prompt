

use iced::theme::Theme;
use iced::widget::scrollable::RelativeOffset;
use iced::widget::{button, column, container, keyed_column, scrollable, text, text_input};
use iced::window::{self, Level};
use iced::{keyboard, Padding};
use iced::{Application, Element};
use iced::{Command, Length, Settings, Size, Subscription};

use once_cell::sync::Lazy;
use uuid::Uuid;

mod core;
mod gui;

use core::cmds;
use core::cmds::{Cmd, Cmds, FilteredCmds};
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
    Selected(Uuid),
}

#[derive(Debug)]
enum Commands {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    cmds: Cmds,
    filtered_cmds: Option<FilteredCmds>,
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
                                cmds: Cmds::from_string(data.value),
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
                    state.filtered_cmds = Some(state.cmds.filter_by_value(&value));
                    state.input_value = value;
                    state.selection = CommandSelection::Initial;

                    Command::none()
                }
                Message::Select(amount) => {
                    let cmds = match &state.filtered_cmds {
                        Some(filtered_cmds) => state.cmds.clone().with_filtered_order(filtered_cmds),
                        None => state.cmds.clone(),
                    };

                    let selection_index = match state.selection {
                        CommandSelection::Initial => 0,
                        CommandSelection::Selected(selected_id) => cmds.order.iter().position(|&id| id == selected_id).unwrap()
                    };

                    let next_index: usize = num::clamp(selection_index as i32 + amount, 0, cmds.order.len() as i32) as usize;

                    let selected_cmd = cmds.get_by_index(next_index);

                    let selection = match selected_cmd.clone() {
                        Some(cmd) => CommandSelection::Selected(*Cmd::uuid(&cmd)),
                        None => CommandSelection::Initial,
                    };

                    let total_cmds = (cmds.order.len() - 1) as f32;

                    state.selection = selection;

                    match selected_cmd {
                        None => scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                        Some(_) => {
                            let offset = (1.0 / (total_cmds)) * selection_index as f32;
                            println!("{offset}");
                            scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset { x: 0.0, y: offset })
                        }
                    }
                }
                Message::Submit => {
                    let id = match state.selection {
                        CommandSelection::Initial => Some(state.cmds.order[0]),
                        CommandSelection::Selected(selected_id) => Some(selected_id),
                    };

                    match id.and_then(|id| state.cmds.cmds.get(&id)) {
                        Some(cmd) => {
                            let value = cmds::Cmd::value(cmd);
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

        let cmds = match &state.filtered_cmds {
            Some(filtered_cmds) => state.cmds.clone().with_filtered_order(filtered_cmds),
            None => state.cmds.clone(),
        };

        let selection = &state.selection;

        let input = text_input("Your prompt", input_value)
            .id(INPUT_ID.clone())
            .style(TextInput::Default)
            .on_submit(Message::Submit)
            .on_input(Message::InputChanged)
            .padding(Padding::from([15., DEFAULT_BORDER_RADIUS + 10.]))
            .size(15.);

        let filtered_items_len = cmds.order.len();

        let items = cmds.map(|i, id, cmd| {
            let value = cmds::Cmd::value(cmd).clone();
            let button_position = match i {
                0 => ButtonPosition::Top,
                _ if i == filtered_items_len - 1 => ButtonPosition::Bottom,
                _ => ButtonPosition::Default,
            };

            let button_style = match (selection, i) {
                (CommandSelection::Initial, 0) => Button::Focused(button_position),
                (CommandSelection::Selected(selected_id), _) if selected_id == id => {
                    Button::Focused(button_position)
                }
                _ => Button::Primary(button_position),
            };

            (
                *id,
                button(
                    container(text(value).line_height(1.25))
                        .height(cmds::SIMPLE_CMD_HEIGHT)
                        .center_y(),
                )
                .style(button_style)
                .width(Length::Fill)
                .into(),
            )
        });

        let cmds = keyed_column(items).spacing(1).into();

        let content: Element<_> = match cmds {
            Some(el) => scrollable(container(el).padding(iced::Padding::from([
                0.,
                10. + crate::gui::style::DEFAULT_BORDER_RADIUS,
                0.,
                0.,
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

        container(column![input, container(content).padding(10)])
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

fn filter_matches(items: &[cmds::Cmd], substring: &str) -> Vec<cmds::Cmd> {
    items
        .iter()
        .filter_map(|cmd| {
            let value = cmds::Cmd::value(cmd);
            let is_match = value.to_lowercase().contains(&substring.to_lowercase());

            if is_match {
                Some(cmd)
            } else {
                None
            }
        })
        .cloned()
        .collect()
}
