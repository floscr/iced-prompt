use std::collections::HashMap;
use std::process;

use levenshtein::levenshtein;
use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;
use std::fmt::{self};

use uuid::Uuid;

// Constants -------------------------------------------------------------------

pub const SIMPLE_CMD_HEIGHT: f32 = 28.;

// Types -----------------------------------------------------------------------

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct ShellProperties {
    pub command: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CommandKind {
    #[default]
    Initial,
    Shell(ShellProperties),
    // Error(CommandError),
}

#[derive(Debug, PartialEq)]
pub enum CommandResultError {
    // IdNotFound(Uuid),
    FailedWithCode(String, i32),
    ExecutionFailed(String),
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Items<T> {
    pub items: HashMap<Uuid, T>,
    pub order: Vec<Uuid>,
}

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum ActionKind {
    #[default]
    #[serde(alias = "exit")]
    Exit,
    #[serde(alias = "next")]
    Next,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct Command {
    pub value: String,
    #[serde(default, alias = "shell", deserialize_with = "deserialize_kind")]
    pub kind: CommandKind,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub action: ActionKind,
    #[serde(default, deserialize_with = "Items::deserialize")]
    pub items: Items<Command>,
}

#[cfg(test)]
mod type_tests {
    use crate::s;
    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, Items, ShellProperties};

    #[test]
    pub fn it_works() {
        let command_uuid = Uuid::new_v4();

        let command = Command {
            value: s!("Commands"),
            items: Items {
                items: HashMap::from([
                    (
                        command_uuid,
                        Command {
                            value: s!("ls"),
                            kind: CommandKind::Shell(ShellProperties { command: s!("ls") }),
                            action: ActionKind::Next,
                            ..Command::default()
                        },
                    ),
                    (
                        command_uuid,
                        Command {
                            value: s!("ls"),
                            kind: CommandKind::Shell(ShellProperties { command: s!("ls") }),
                            action: ActionKind::Exit,
                            ..Command::default()
                        },
                    ),
                ]),
                order: vec![command_uuid],
            },
            ..Command::default()
        };
        assert_eq!(command.value, "Commands");
    }
}

// Deserialization -------------------------------------------------------------

fn parse_command_or_exit(json_str: &str) -> Option<Command> {
    match serde_json::from_str(json_str) {
        Ok(cmd) => Some(cmd),
        Err(err) => {
            println!("{:#?}", err);
            std::process::exit(1);
        }
    }
}

impl<'de> Deserialize<'de> for CommandKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandKindVisitor;

        impl<'de> Visitor<'de> for CommandKindVisitor {
            type Value = CommandKind;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid CommandKind")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CommandKind, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut shell_command = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "shell" {
                        let command: String = map.next_value()?;
                        shell_command = Some(ShellProperties { command });
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                Ok(shell_command.map_or(CommandKind::Initial, CommandKind::Shell))
            }
        }

        const FIELDS: &[&str] = &["shell"];
        deserializer.deserialize_struct("CommandKind", FIELDS, CommandKindVisitor)
    }
}

// Deserialize kind from a simple value:
// {"shell": "ls"} -> CommandKind::SyncShellCommand { command: "ls" }
fn deserialize_kind<'de, D>(deserializer: D) -> Result<CommandKind, D::Error>
where
    D: Deserializer<'de>,
{
    let obj = Value::deserialize(deserializer)?;
    match obj {
        Value::String(shell_string) => Ok(CommandKind::Shell(ShellProperties {
            command: shell_string.to_owned(),
        })),
        _ => Ok(CommandKind::Initial),
    }
}

// Deserialize items from a flat array to Items<Command>
impl<'de> Deserialize<'de> for Items<Command> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemsVisitor;

        impl<'de> Visitor<'de> for ItemsVisitor {
            type Value = Items<Command>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of commands")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Items<Command>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = HashMap::new();
                let mut order = Vec::new();

                while let Some(command) = seq.next_element::<Command>()? {
                    let uuid = Uuid::new_v4(); // Generate a new UUID for each item
                    order.push(uuid);
                    items.insert(uuid, command);
                }

                Ok(Items { items, order })
            }
        }

        deserializer.deserialize_seq(ItemsVisitor)
    }
}

#[cfg(test)]
mod deserialize_tests {
    use super::{Command, CommandKind, ShellProperties};
    use crate::s;

    #[test]
    fn deserializes_system_types_json() {
        let data = include_str!("../../data/system_types_simple.json");

        let v: Command = serde_json::from_str(data).unwrap();

        assert_eq!(v.value, "Commands");
        assert_eq!(v.items.order.len(), 1);
        let (_, cmd) = v.get_child_command_by_index(0).unwrap();
        assert_eq!(cmd.value, "List files: ~");
    }

    #[test]
    fn deserializes_user_friendly_json() {
        let data = include_str!("../../data/user_friendly_simple.json");

        let v: Command = serde_json::from_str(data).unwrap();

        assert_eq!(v.value, "Commands");
        assert_eq!(v.items.order.len(), 1);
        let (_, cmd) = v.get_child_command_by_index(0).unwrap();
        assert_eq!(cmd.value, "List files: ~");
        assert_eq!(
            cmd.kind,
            CommandKind::Shell(ShellProperties {
                command: s!("bb ./scripts/src/file_explorer.clj ~")
            })
        );
    }

    #[test]
    fn deserialize_command_with_defaults() {
        let data = r#"{
    "type": "Command",
    "value": "Commands",
    "kind": "Initial",
    "items": []
}"#;

        let v: Command = serde_json::from_str(data).unwrap();

        assert_eq!(v.value, "Commands");
        assert_eq!(v.items.order.len(), 0);
    }
}

// Impl ------------------------------------------------------------------------

impl CommandKind {
    pub fn sync_execute(shell_command: ShellProperties) -> Result<String, CommandResultError> {
        let output = process::Command::new("sh")
            .arg("-c")
            .arg(shell_command.command)
            .output();

        match output {
            Ok(output) => {
                match output.status.code() {
                    // Success
                    Some(0) => {
                        let result = String::from_utf8_lossy(&output.stdout)
                            .to_string()
                            .trim_end()
                            .to_string();

                        Ok(result)
                    }
                    // Failed with specific code
                    Some(code) => Err(CommandResultError::FailedWithCode(
                        String::from_utf8_lossy(&output.stderr).to_string(),
                        code,
                    )),
                    // Process terminated by a signal
                    None => Err(CommandResultError::ExecutionFailed(
                        "Command terminated by signal".to_string(),
                    )),
                }
            }
            // Some other error
            Err(e) => Err(CommandResultError::ExecutionFailed(e.to_string())),
        }
    }
}

impl Command {
    pub fn map_filter_items<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(usize, &Uuid, &Command) -> Option<T>,
    {
        self.items
            .order
            .iter()
            .enumerate()
            .filter_map(|(index, id)| self.items.items.get(id).and_then(|cmd| f(index, id, cmd)))
            .collect()
    }

    pub fn filter_items_by_value(&self, substring: &str) -> Vec<Uuid> {
        let mut items: Vec<(usize, Uuid)> = self.map_filter_items(|_, id, command| {
            let value = &command.value;
            let matches_value = value.to_lowercase().contains(&substring.to_lowercase());
            if matches_value {
                Some((levenshtein(value, substring), *id))
            } else {
                None
            }
        });
        items.sort_by(|(a, _), (b, _)| a.cmp(b));

        items.into_iter().map(|(_, v)| v).collect()
    }

    pub fn index_of_item_with_id(&self, id: Uuid) -> Option<usize> {
        self.items.order.iter().position(|&order_id| order_id == id)
    }

    pub fn get_child_command_by_index(&self, index: usize) -> Option<(Uuid, Command)> {
        let id = self.items.order.get(index);

        let item = id.and_then(|id| self.items.items.get(id).cloned());

        match (id, item) {
            (Some(id), Some(item)) => Some((*id, item)),
            _ => None,
        }
    }

    pub fn with_order(&self, order: Vec<Uuid>) -> Command {
        Command {
            items: Items {
                order,
                ..self.items.clone()
            },
            ..self.clone()
        }
    }

    pub fn command_kind_height(&self) -> f32 {
        SIMPLE_CMD_HEIGHT
            // Spacing
            + 1.
            // Figure out what this value is
            + 10.
    }

    pub fn scroll_offset_at_index(&self, index: usize) -> f32 {
        let ids = &self.items.order[..index];
        let mut offset = 0.;
        for id in ids {
            offset += &self
                .items
                .items
                .get(id)
                .map(Command::command_kind_height)
                .unwrap_or(0.)
        }
        offset
    }

    pub fn execute(&self) -> Result<String, CommandResultError> {
        match &self.kind {
            CommandKind::Initial => Ok(self.value.clone()),
            CommandKind::Shell(shell_command) => CommandKind::sync_execute(shell_command.clone()),
        }
    }

    pub fn execute_action(&self) -> Option<Command> {
        let result = self.execute();

        match &self.action {
            ActionKind::Exit => {
                match result {
                    Ok(output) => {
                        println!("{:#?}", output);
                        std::process::exit(0)
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                        std::process::exit(1);
                    }
                };
            }
            ActionKind::Next => {
                if let Ok(json_str) = result {
                    parse_command_or_exit(&json_str)
                } else {
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::s;

    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, CommandResultError, Items, ShellProperties};

    fn make_test_command() -> Command {
        let command_a_uuid = Uuid::new_v4();
        let command_b_uuid = Uuid::new_v4();

        Command {
            value: s!("Commands"),
            items: Items {
                items: HashMap::from([
                    (
                        command_a_uuid,
                        Command {
                            value: s!("ls"),
                            kind: CommandKind::Shell(ShellProperties { command: s!("ls") }),
                            action: ActionKind::Next,
                            ..Command::default()
                        },
                    ),
                    (
                        command_b_uuid,
                        Command {
                            value: s!("pwd"),
                            kind: CommandKind::Shell(ShellProperties { command: s!("pwd") }),
                            action: ActionKind::Exit,
                            ..Command::default()
                        },
                    ),
                    (
                        command_b_uuid,
                        Command {
                            value: s!("pwd"),
                            kind: CommandKind::Shell(ShellProperties { command: s!("pwd") }),
                            action: ActionKind::Exit,
                            ..Command::default()
                        },
                    ),
                ]),
                order: vec![command_a_uuid, command_b_uuid],
            },
            ..Command::default()
        }
    }

    #[test]
    fn maps_over_items_to_extract_values() {
        let command = make_test_command();

        let command_values = command.map_filter_items(|_, _, cmd| Some(cmd.value.clone()));

        assert_eq!(command_values, vec![s!("ls"), s!["pwd"]]);
    }

    #[test]
    fn filters_by_value() {
        let command = make_test_command();
        let target_uuid = &command.items.order[0];

        let command_values = command.filter_items_by_value("ls");

        assert_eq!(command_values.len(), 1);
        assert_eq!(command_values[0], target_uuid.clone());
    }

    #[test]
    fn execute_successful_command() {
        let command = Command {
            value: s!("Success"),
            kind: CommandKind::Shell(ShellProperties {
                command: s!("echo \"Success\""),
            }),
            ..Command::default()
        };

        let result = command.execute();
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, "Success");
    }

    fn execute_failing_command() {
        let command = Command {
            value: s!("Fail"),
            kind: CommandKind::Shell(ShellProperties {
                command: s!("echo \"Fail\"; exit 1"),
            }),
            ..Command::default()
        };

        let result = command.execute();
        assert!(result.is_err());

        let error = result.unwrap_err();
        let expected_error = CommandResultError::FailedWithCode(String::from("Fail\n"), 1);
        assert_eq!(error, expected_error);
    }
}
