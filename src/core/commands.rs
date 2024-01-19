use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ModeProperties {
    pub title: Option<String>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShellCommandProperties {
    pub title: Option<String>,
    pub command: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    #[default]
    Unknown,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CommandError {
    pub error_kind: ErrorKind,
    pub error: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CommandKind {
    #[default]
    StdOutCommand(ModeProperties),
    SyncShellCommand(ShellCommandProperties),
    Error(CommandError),
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ActionKind {
    #[default]
    Exit,
    Next,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Action {
    pub command_kind: CommandKind,
    pub action_kind: ActionKind,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Command {
    pub command_kind: CommandKind,
    pub actions: HashMap<Uuid, Action>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mode {
    pub command_kind: CommandKind,
    pub items: HashMap<Uuid, Command>,
    pub order: Vec<Uuid>,
}
