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

// use core::commands::Command;
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
        let mode = Mode {
            kind: ModeKind::SyncShellCommand(mode::ShellCommandProperties {
                command: String::from("ls"),
            }),
            ..Mode::default()
        }
        .execute();
        let state = State {
            mode,
            ..State::default()
        };
        (
            LoadingState::Loaded(state),
            // Command::none(),
            // Command::perform(Mode::execute(mode), Message::IoLoaded),
            text_input::focus(INPUT_ID.clone()),
        )
        // (
        //     Commands::Loading,
        //     Command::perform(PromptData::load(), Message::IoLoaded),
        // )
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
                    state.filtered_cmds = Some(state.mode.filter_by_value(&value));
                    state.input_value = value;
                    state.selection = CommandSelection::Initial;

                    scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START)
                }
                Message::Select(amount) => {
                    let cmds = match &state.filtered_cmds {
                        Some(filtered_cmds) => {
                            state.mode.clone().with_filtered_order(filtered_cmds)
                        }
                        None => state.mode.clone(),
                    };

                    let selection_index = match state.selection {
                        CommandSelection::Initial => 0,
                        CommandSelection::Selected(selected_id) => {
                            cmds.order.iter().position(|&id| id == selected_id).unwrap()
                        }
                    };

                    let next_index: usize =
                        num::clamp(selection_index as i32 + amount, 0, cmds.order.len() as i32)
                            as usize;

                    let selected_cmd = cmds.get_by_index(next_index);

                    let selection = match selected_cmd.clone() {
                        Some(cmd) => CommandSelection::Selected(*Item::uuid(&cmd)),
                        None => CommandSelection::Initial,
                    };

                    state.selection = selection;

                    match selected_cmd {
                        None => scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                        Some(_) => {
                            let scroll_offset =
                                cmds.scroll_offset_at_index(next_index) - state.scrollable_offset.y;
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
                    let id = match state.selection {
                        CommandSelection::Initial => Some(state.mode.order[0]),
                        CommandSelection::Selected(selected_id) => Some(selected_id),
                    };

                    match id.and_then(|id| state.mode.items.get(&id)) {
                        Some(cmd) => {
                            let value = mode::Item::value(cmd);
                            println!("{}", value);
                            std::process::exit(0)
                        }
                        _ => std::process::exit(1),
                    }
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

        let cmds = match &state.filtered_cmds {
            Some(filtered_cmds) => state.mode.clone().with_filtered_order(filtered_cmds),
            None => state.mode.clone(),
        };

        let selection = &state.selection;

        let filtered_items_len = cmds.order.len();

        let items = cmds.map(|i, id, cmd| {
            let value = mode::Item::value(cmd).clone();
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
                        .height(mode::SIMPLE_CMD_HEIGHT)
                        .center_y(),
                )
                .style(button_style)
                .width(Length::Fill)
                .into(),
            )
        });

        let cmds = keyed_column(items)
            .spacing(1)
            .padding(iced::Padding::from([
                0.,
                10. + crate::gui::style::DEFAULT_BORDER_RADIUS + 10.,
                0.,
                10.,
            ]))
            .into();

        let content: Element<_> = match cmds {
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
