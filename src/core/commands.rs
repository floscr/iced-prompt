use std::collections::HashMap;
use std::{io, process};

use uuid::Uuid;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShellCommandProperties {
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
    StdOutCommand,
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
pub enum ActionResultKind {
    #[default]
    Ignore,
    Exit(Option<String>),
    Next(Mode),
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Action {
    pub title: Option<String>,
    pub command_kind: CommandKind,
    pub action_kind: ActionKind,
}

fn execute_sync_shell_command(command: String) -> io::Result<process::Output> {
    process::Command::new("sh")
        .arg("-c")
        .arg(command.clone())
        .output()
}

pub fn execute_command_action(command: CommandKind, action: ActionKind) -> ActionResultKind {
    match self.command_kind {
        CommandKind::StdOutCommand => ActionResultKind::Exit(self.title),
        CommandKind::SyncShellCommand(properties) => {
            let output = execute_sync_shell_command(properties.command)
                .expect("Failed to execute command");

            match (output.status.code()) {
                Some(0) => {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    match action_kind
                }
                Some(_code) => {
                    Mode {
                        kind: ModeKind::SyncShellCommand(ShellCommandProperties { command }),
                        ..self
                    }
                    // let result = String::from_utf8_lossy(&output.stderr);
                    // eprintln!("Error executing command (exit code {}):\n{}", code, result);
                }
                None => {
                    Mode {
                        kind: ModeKind::SyncShellCommand(ShellCommandProperties { command }),
                        ..self
                    }
                    // // The command was terminated by a signal or other unexpected event
                    // eprintln!("Command terminated unexpectedly");
                }
            }
        }
        _ => ActionResultKind::Ignore,
    }
}


// impl Action {
// }

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Command {
    pub title: Option<String>,
    pub command_kind: CommandKind,
    pub action_kind: ActionKind,
    pub actions: HashMap<Uuid, Action>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mode {
    pub title: Option<String>,
    pub command_kind: CommandKind,
    pub items: HashMap<Uuid, Command>,
    pub order: Vec<Uuid>,
}
