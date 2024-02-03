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

use core::commands::{Command, SIMPLE_CMD_HEIGHT};
use core::history::History;
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
    selection: CommandSelection,
    scrollable_offset: AbsoluteOffset,
}

#[derive(Debug, Clone)]
enum Message {
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
        let data = include_str!("../data/system_types_simple.json");

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
                // match message {
                //     Message::IoLoaded(result) => {
                //         *self = match result {
                //             Some(_data) => LoadingState::Loaded(State { ..State::default() }),
                //             None => LoadingState::Loaded(State::default()),
                //         };
                //     }
                //     _ => {}
                // }
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

                            let scroll_offset = filtered_cmds.scroll_offset_at_index(next_index);

                            filtered_cmds
                                .get_child_command_by_index(next_index)
                                .map(|(id, _)| (id, scroll_offset))
                        }
                        _ => None,
                    };

                    let next_selection = match &selected_command_and_index {
                        Some((id, _)) => CommandSelection::Selected(*id),
                        None => CommandSelection::Initial,
                    };

                    state.selection = next_selection;

                    match &selected_command_and_index {
                        None => scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                        Some((_, scroll_offset)) => {
                            let next_scroll_offset = scroll_offset - state.scrollable_offset.y;

                            scrollable::scroll_to(
                                SCROLLABLE_ID.clone(),
                                AbsoluteOffset {
                                    x: 0.0,
                                    y: next_scroll_offset,
                                },
                            )
                        }
                    }
                }
                Message::Submit => {
                    if let Some(cmds) = state.history.head() {
                        let id = match &state.selection {
                            CommandSelection::Initial => cmds.items.order[0],
                            CommandSelection::Selected(selected_id) => *selected_id,
                        };

                        let command = cmds.items.items.get(&id);

                        match command {
                            Some(cmd) => {
                                let value = &cmd.value;
                                println!("{}", value);
                                std::process::exit(0)
                            }
                            _ => std::process::exit(1),
                        }
                    }

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
                            .height(SIMPLE_CMD_HEIGHT)
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
