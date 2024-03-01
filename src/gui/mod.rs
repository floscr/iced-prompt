pub mod colors;
pub mod fonts;
pub mod icons;
pub mod style;

use core::fmt;
use iced::keyboard::{KeyCode, Modifiers};
use iced::theme::Theme;
use iced::widget::scrollable::{AbsoluteOffset, RelativeOffset, Viewport};
use iced::widget::{
    button, column, container, horizontal_rule, row, scrollable, svg, text, text_input,
};
use std::sync::{Arc, Mutex};

use iced::window::{self, Level};
use iced::{font, subscription, Alignment, Event, Padding};
use iced::{Application, Element};
use iced::{Length, Settings, Subscription};

use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::core::commands::{Command, CommandResultError, SIMPLE_CMD_HEIGHT};
use crate::core::history::History;
use fonts::ROBOTO_BYTES;
use style::DEFAULT_BORDER_RADIUS;
use style::{footer_container_style, get_svg_style};

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum AppError {
    Iced(iced::Error),
    NoCommandFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Iced(err) => write!(f, "Iced error: {}", err),
            AppError::NoCommandFound => write!(f, "No command found"),
        }
    }
}

pub fn main(cmd: Command) -> Result<Command, AppError> {
    let result = Arc::new(Mutex::new(None));

    let window_result = LoadingState::run(Settings {
        window: window::Settings {
            size: (700, 500),
            position: window::Position::Centered,
            transparent: true,
            decorations: false,
            resizable: false,
            level: Level::AlwaysOnTop,
            ..window::Settings::default()
        },
        flags: ApplicationFlags {
            cmd,
            result: result.clone(),
        },
        default_font: fonts::ROBOTO,
        antialiasing: true,
        ..Settings::default()
    });

    let result_lock = result.lock().unwrap();
    let cmd = result_lock.clone();

    match window_result {
        Ok(_) => match cmd {
            Some(c) => Ok(c),
            None => Err(AppError::NoCommandFound),
        },
        Err(err) => Err(AppError::Iced(err)),
    }
}

#[derive(Debug, Default)]
enum Selection {
    #[default]
    Initial,
    Selected(Uuid),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    history: History,
    filter: Option<Vec<Uuid>>,
    selection: Selection,
    scrollable_offset: AbsoluteOffset,
    result: Arc<Mutex<Option<Command>>>,
    jobs: HashMap<Uuid, ()>,
}

#[derive(Debug)]
enum LoadingState {
    Loaded(State),
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ToggleFullscreen(window::Mode),
    Exit(i32),
    Select(i32),
    Submit(Option<Uuid>),
    OnScroll(Viewport),
    HistoryBackwards,
    FontLoaded(Result<(), font::Error>),
    PushHistory(Command),
}

impl State {
    fn navigate(&mut self, history: History) -> iced::Command<Message> {
        self.filter = None;
        self.selection = Selection::Initial;
        self.input_value = "".to_string();
        self.history = history;

        iced::Command::batch(vec![
            text_input::focus(INPUT_ID.clone()),
            scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
        ])
    }
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

#[derive(Default)]
struct ApplicationFlags {
    cmd: Command,
    result: Arc<Mutex<Option<Command>>>,
}

impl Application for LoadingState {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ApplicationFlags;

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(flags: ApplicationFlags) -> (LoadingState, iced::Command<Message>) {
        let state = State {
            history: History::default().push(flags.cmd),
            result: flags.result.clone(),
            ..State::default()
        };
        (
            LoadingState::Loaded(state),
            iced::Command::batch(vec![
                font::load(ROBOTO_BYTES).map(Message::FontLoaded),
                text_input::focus(INPUT_ID.clone()),
            ]),
        )
    }

    fn title(&self) -> String {
        "Iced Query".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match self {
            LoadingState::Loaded(state) => match message {
                Message::OnScroll(viewport) => {
                    state.scrollable_offset = viewport.absolute_offset();
                    iced::Command::none()
                }
                Message::HistoryBackwards => {
                    let prev_history = state.history.clone().pop_with_minimum();
                    state.navigate(prev_history)
                }
                Message::PushHistory(command) => {
                    let history = &state.history;
                    let next_history = history.clone().push(command);
                    state.jobs.clear();
                    state.navigate(next_history)
                }
                Message::InputChanged(value) => {
                    if value.is_empty() {
                        state.filter = None;
                    } else {
                        state.filter = state
                            .history
                            .head()
                            .map(|cmd| Command::filter_items_by_value(&cmd, &value));
                    }
                    state.input_value = value;
                    state.selection = Selection::Initial;

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
                                Selection::Initial => 0,
                                Selection::Selected(id) => {
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
                        Some((id, _)) => Selection::Selected(*id),
                        None => Selection::Initial,
                    };

                    state.selection = next_selection;

                    match &selected_command_and_index {
                        None => scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                        Some((_, scroll_offset)) => scrollable::scroll_to(
                            SCROLLABLE_ID.clone(),
                            AbsoluteOffset {
                                x: 0.0,
                                y: *scroll_offset,
                            },
                        ),
                    }
                }
                Message::Submit(maybe_id) => {
                    let history = &state.history;
                    let filter = &state.filter;

                    let opt_cmds = history.head();

                    if opt_cmds.is_none() {
                        return iced::Command::none();
                    }
                    let cmds = opt_cmds.unwrap();

                    let id = if let Some(maybe_id) = maybe_id {
                        maybe_id
                    } else {
                        match &state.selection {
                            Selection::Initial => {
                                let order = filter.clone().unwrap_or(cmds.items.order);
                                order[0]
                            }
                            Selection::Selected(selected_id) => *selected_id,
                        }
                    };

                    let opt_command = cmds.items.items.get(&id);
                    if opt_command.is_none() {
                        return iced::Command::none();
                    }
                    let command = opt_command.unwrap();

                    match command.action {
                        // Next: Try to push result on the history stack
                        crate::core::commands::ActionKind::Next => {
                            let command_for_async = command.clone();
                            state.jobs.insert(id, ());

                            iced::Command::perform(
                                async { command_for_async.execute() },
                                |io_output| {
                                    let cmd: Result<Command, CommandResultError> =
                                        io_output.and_then(|s| Command::parse(&s));
                                    match cmd {
                                        Ok(c) => Message::PushHistory(c),
                                        Err(err) => {
                                            println!("{:#?}", err);
                                            std::process::exit(1);
                                        }
                                    }
                                },
                            )
                        }
                        // Close window & save command so it can be further processed
                        _ => {
                            let mut result = state.result.lock().unwrap();
                            *result = Some(command.clone());

                            window::close()
                        }
                    }
                }
                Message::Exit(exit_code) => std::process::exit(exit_code),
                _ => iced::Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        use crate::gui::style::{default_container_style, Button, Rule, TextInput};

        let _default_state = State::default();
        let LoadingState::Loaded(state) = self;
        let input_value = &state.input_value;
        let history = &state.history;
        let selection = &state.selection;

        let current_cmds = history.head().unwrap_or_default();
        let order: &Vec<Uuid> = &state.filter.clone().unwrap_or(current_cmds.items.order);

        let items = order
            .iter()
            .enumerate()
            .map(|(idx, id)| {
                let cmd = current_cmds.items.items.get(id).unwrap();
                let value = &cmd.value;
                let icon = &cmd.icon;

                let button_style = match (selection, idx) {
                    (Selection::Initial, 0) => Button::Focused,
                    (Selection::Selected(selected_id), _) if selected_id == id => Button::Focused,
                    _ => Button::Primary,
                };

                let text_value = text(value).line_height(1.25);

                let icon_element = match icon {
                    Some(icon_string) => match icon_string.as_str() {
                        "Directory" => Some(icons::DIRECTORY.clone()),
                        "File" => Some(icons::FILE.clone()),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|svg_icon| svg(svg_icon).width(20.).height(20.).style(get_svg_style()));

                let button_content = match icon_element {
                    Some(icon_el) => row![icon_el, text_value].spacing(5),
                    _ => row![text_value],
                };

                button(
                    container(button_content)
                        .height(SIMPLE_CMD_HEIGHT)
                        .center_y(),
                )
                .style(iced::theme::Button::Custom(Box::new(button_style)))
                .width(Length::Fill)
                .on_press(Message::Submit(Some(*id)))
                .into()
            })
            .collect();

        let cmds_column = column(items)
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
                .style(crate::gui::style::Scrollable::Default)
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
                .on_submit(Message::Submit(None))
                .on_input(Message::InputChanged)
                .padding(Padding::from([15., DEFAULT_BORDER_RADIUS + 10.]))
                .size(15.),
            horizontal_rule(1).style(Rule::Default),
        ];

        let footer: Element<_> = column![
            horizontal_rule(1).style(Rule::Default),
            container(text(current_cmds.value).size(13))
                .style(footer_container_style())
                .center_y()
                .height(Length::Fill)
                .padding(iced::Padding::from([0, 10])),
        ]
        .height(35)
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
                .style(default_container_style())
                .center_y(),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers,
                key_code,
            }) => match (key_code, modifiers) {
                (KeyCode::Backspace, Modifiers::SHIFT) => Some(Message::HistoryBackwards),
                (KeyCode::Tab, Modifiers::SHIFT) => Some(Message::HistoryBackwards),
                (KeyCode::Escape, _) => Some(Message::Exit(0)),
                (KeyCode::Up, Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (KeyCode::Down, Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                (KeyCode::Up, _) | (KeyCode::P, Modifiers::CTRL) => Some(Message::Select(-1)),
                (KeyCode::Down, _) | (KeyCode::N, Modifiers::CTRL) => Some(Message::Select(1)),
                _ => None,
            },
            _ => None,
        })
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(Box::new(ApplicationStyle {}))
    }
}
