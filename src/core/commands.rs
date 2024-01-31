use std::collections::HashMap;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct ShellCommandProperties {
    pub command: String,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub enum CommandKind {
    #[default]
    Initial,
    SyncShellCommand(ShellCommandProperties),
    // Error(CommandError),
}

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Items<T> {
    pub items: HashMap<Uuid, T>,
    pub order: Vec<Uuid>,
}

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum ActionKind {
    #[default]
    Exit,
    Next,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct Command {
    pub value: String,
    #[serde(default)]
    pub kind: CommandKind,
    #[serde(default)]
    pub action: ActionKind,
    #[serde(default, flatten)]
    pub items: Option<Items<Command>>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, Items, ShellCommandProperties};

    macro_rules! s {
        ($s:expr) => {
            $s.to_string()
        };
    }

    #[test]
    pub fn it_works() {
        let command_uuid = Uuid::new_v4();

        let command = Command {
            value: s!("Commands"),
            items: Some(Items {
                items: HashMap::from([
                    (
                        command_uuid,
                        Command {
                            value: s!("ls"),
                            kind: CommandKind::SyncShellCommand(ShellCommandProperties {
                                command: s!("ls"),
                            }),
                            action: ActionKind::Next,
                            ..Command::default()
                        },
                    ),
                    (
                        command_uuid,
                        Command {
                            value: s!("ls"),
                            kind: CommandKind::SyncShellCommand(ShellCommandProperties {
                                command: s!("ls"),
                            }),
                            action: ActionKind::Exit,
                            ..Command::default()
                        },
                    ),
                ]),
                order: vec![command_uuid],
            }),
            ..Command::default()
        };
        assert_eq!(command.value, "Commands");
    }

    #[test]
    fn it_deserializes() {
        let data = r#"{
            "type": "Commands",
            "value": "Commands",
            "kind": "Initial"
        }"#;

        let v: Command = serde_json::from_str(data).unwrap();

        assert_eq!(v.value, "Commands");
    }
}
