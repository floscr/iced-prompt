use iced::theme::Theme;
use iced::widget::scrollable::{AbsoluteOffset, RelativeOffset, Viewport};
use iced::widget::{
    button, column, container, horizontal_rule, keyed_column, row, scrollable, text, text_input,
};
use iced::window::{self, Level};
use iced::{keyboard, Alignment, Padding};
use iced::{Application, Element};
use iced::{Length, Settings, Size, Subscription};

use once_cell::sync::Lazy;
use uuid::Uuid;

mod core;
mod gui;
mod utils;

use core::commands::{self, Command};
use core::history::History;
use core::mode::{self, FilteredItems, Item, Mode, ModeKind, ShellCommandProperties};
use gui::style::DEFAULT_BORDER_RADIUS;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    LoadingState::run(Settings {
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
enum LoadingState {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    history: History,
    filter: Option<Vec<Uuid>>,
    mode: Mode,
    filtered_cmds: Option<FilteredItems>,
    selection: CommandSelection,
    scrollable_offset: AbsoluteOffset,
}

#[derive(Debug, Clone)]
enum Message {
    IoLoaded(Option<PromptData>),
    InputChanged(String),
    ToggleFullscreen(window::Mode),
    Exit,
    Select(i32),
    Submit,
    OnScroll(Viewport),
    Execute,
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

impl Application for LoadingState {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (LoadingState, iced::Command<Message>) {
        let data = r#"{
    "value": "Commands",
    "kind": "Initial",
    "action": "Exit",
    "items": [
        {
        "value": "ls",
        "kind": {
            "SyncShellCommand": {
                "command": "ls"
            }
        },
        "action": "Next"
    },
                {
        "value": "pwd",
        "kind": {
            "SyncShellCommand": {
                "command": "ls"
            }
        },
        "action": "Next"
    }

    ]
}"#;

        let cmd: Command = serde_json::from_str(data).unwrap();

        let state = State {
            history: History::default().push(cmd),
            ..State::default()
        };
        (
            LoadingState::Loaded(state),
            text_input::focus(INPUT_ID.clone()),
        )
    }

    fn title(&self) -> String {
        "Iced Query".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match self {
            LoadingState::Loading => {
                #[allow(clippy::single_match)]
                match message {
                    Message::IoLoaded(result) => {
                        *self = match result {
                            Some(data) => LoadingState::Loaded(State {
                                mode: Mode::from_string(data.value),
                                ..State::default()
                            }),
                            None => LoadingState::Loaded(State::default()),
                        };
                    }
                    _ => {}
                }

                text_input::focus(INPUT_ID.clone())
            }
            LoadingState::Loaded(state) => match message {
                Message::OnScroll(viewport) => {
                    state.scrollable_offset = viewport.absolute_offset();
                    iced::Command::none()
                }
                Message::InputChanged(value) => {
                    state.filter = state
                        .history
                        .head()
                        .map(|cmd| Command::filter_items_by_value(&cmd, &value));
                    state.input_value = value;
                    state.selection = CommandSelection::Initial;

                    scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START)
                }
                Message::Select(amount) => {
                    let selected_command_and_index = match state.history.head() {
                        Some(cmds) => {
                            let filtered_cmds = match &state.filter {
                                Some(order) => cmds.with_order(order.clone()),
                                _ => cmds,
                            };

                            let selection_index: usize = match state.selection {
                                CommandSelection::Initial => 0,
                                CommandSelection::Selected(id) => {
                                    let i = &filtered_cmds.index_of_item_with_id(id).unwrap_or(0);
                                    *i
                                }
                            };

                            // Shift the index by the given `amount`
                            let cmds_len = &filtered_cmds.items.order.len();
                            let next_index: usize =
                                num::clamp(selection_index as i32 + amount, 0, *cmds_len as i32)
                                    as usize;

                            filtered_cmds
                                .get_child_command_by_index(next_index)
                                .map(|(id, cmd)| (id, next_index, cmd))
                        }
                        _ => None,
                    };

                    let next_selection = match &selected_command_and_index {
                        Some((id, _, _)) => CommandSelection::Selected(*id),
                        None => CommandSelection::Initial,
                    };
                    state.selection = next_selection;

                    // iced::Command::none()

                    match &selected_command_and_index {
                        None => scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                        Some((_, idx, cmd)) => {
                            let scroll_offset =
                                cmd.scroll_offset_at_index(*idx) - state.scrollable_offset.y;
                            scrollable::scroll_to(
                                SCROLLABLE_ID.clone(),
                                AbsoluteOffset {
                                    x: 0.0,
                                    y: scroll_offset,
                                },
                            )
                        }
                    }
                }
                Message::Submit => {
                    // let id = match state.selection {
                    //     CommandSelection::Initial => Some(state.mode.order[0]),
                    //     CommandSelection::Selected(selected_id) => Some(selected_id),
                    // };

                    // match id.and_then(|id| state.mode.items.get(&id)) {
                    //     Some(cmd) => {
                    //         let value = mode::Item::value(cmd);
                    //         println!("{}", value);
                    //         std::process::exit(0)
                    //     }
                    //     _ => std::process::exit(1),
                    // }
                    iced::Command::none()
                }
                Message::Exit => std::process::exit(0),
                Message::ToggleFullscreen(mode) => window::change_mode(window::Id::MAIN, mode),
                _ => iced::Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        use crate::gui::style::{
            get_item_container_style, Button, ButtonPosition, Rule, TextInput,
        };

        let default_state = State::default();
        let state = match self {
            LoadingState::Loading => &default_state,
            LoadingState::Loaded(state) => state,
        };
        let input_value = &state.input_value;
        let history = &state.history;
        let filter = &state.filter;
        let selection = &state.selection;

        let current_cmds = history.head().unwrap_or_default();

        let filtered_items_len = filter
            .clone()
            .map(|x| x.len())
            .unwrap_or(current_cmds.items.order.len());

        let items = current_cmds.map_filter_items(|i, id, cmd| {
            let value = &cmd.value;

            let matches_value = value.to_lowercase().contains(&input_value.to_lowercase());

            if matches_value {
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

                Some((
                    *id,
                    button(
                        container(text(value).line_height(1.25))
                            .height(mode::SIMPLE_CMD_HEIGHT)
                            .center_y(),
                    )
                    .style(button_style)
                    .width(Length::Fill)
                    .into(),
                ))
            } else {
                None
            }
        });

        let cmds_column = keyed_column(items)
            .spacing(1)
            .padding(iced::Padding::from([
                0.,
                10. + crate::gui::style::DEFAULT_BORDER_RADIUS + 10.,
                0.,
                10.,
            ]))
            .into();

        let content: Element<_> = match cmds_column {
            Some(el) => scrollable(row![el])
                .on_scroll(Message::OnScroll)
                .height(Length::Fill)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Properties::new()
                        .width(10)
                        .margin(8)
                        .scroller_width(8)
                        .alignment(scrollable::Alignment::Start),
                ))
                .id(SCROLLABLE_ID.clone())
                .into(),
            _ => container(text("Nothing found"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
        };

        let input = column![
            text_input("Your prompt", input_value)
                .id(INPUT_ID.clone())
                .style(TextInput::Default)
                .on_submit(Message::Submit)
                .on_input(Message::InputChanged)
                .padding(Padding::from([15., DEFAULT_BORDER_RADIUS + 10.]))
                .size(15.),
            horizontal_rule(1).style(Rule::Default),
        ];

        let footer: Element<_> = column![
            horizontal_rule(1).style(Rule::Default),
            container(text("Footer"))
                .center_y()
                .height(Length::Fill)
                .padding(iced::Padding::from([0, 10])),
        ]
        .height(30)
        .into();

        let wrapper: Element<Message> = column![row![input], content, row![footer]]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .into();

        Element::from(
            container(wrapper)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .padding(1)
                .style(get_item_container_style())
                .center_y(),
        )
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

fn filter_matches(items: &[mode::Item], substring: &str) -> Vec<mode::Item> {
    items
        .iter()
        .filter_map(|cmd| {
            let value = mode::Item::value(cmd);
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
