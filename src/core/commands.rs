use std::collections::HashMap;

use serde::de::{Deserializer, SeqAccess, Visitor};
use serde::Deserialize;
use std::fmt;

use uuid::Uuid;

// Constants -------------------------------------------------------------------

pub const SIMPLE_CMD_HEIGHT: f32 = 28.;

// Types -----------------------------------------------------------------------

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct ShellCommandProperties {
    pub command: String,
}

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum CommandKind {
    #[default]
    Initial,
    SyncShellCommand(ShellCommandProperties),
    // Error(CommandError),
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    #[serde(default, deserialize_with = "Items::deserialize")]
    pub items: Items<Command>,
}
#[cfg(test)]
mod type_tests {
    use crate::s;
    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, Items, ShellCommandProperties};

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
            },
            ..Command::default()
        };
        assert_eq!(command.value, "Commands");
    }
}

// Deserialization -------------------------------------------------------------

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
    use crate::s;
    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, Items, ShellCommandProperties};

    #[test]
    fn deserializes_nested_command() {
        let data = include_str!("../../data/nested_simple.json");

        let v: Command = serde_json::from_str(data).unwrap();

        assert_eq!(v.value, "Commands");
        assert_eq!(v.items.order.len(), 1);
        let (_, cmd) = v.get_child_command_by_index(0).unwrap();
        assert_eq!(cmd.value, "List files: ~");
    }

    #[test]
    fn deserialize_simple_command_with_defaults() {
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
        self.map_filter_items(|_, id, command| {
            let matches_value = command
                .value
                .to_lowercase()
                .contains(&substring.to_lowercase());
            if matches_value {
                Some(*id)
            } else {
                None
            }
        })
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
        SIMPLE_CMD_HEIGHT + 1.
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
}

#[cfg(test)]
mod command_tests {
    use crate::s;

    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{ActionKind, Command, CommandKind, Items, ShellCommandProperties};

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
                            kind: CommandKind::SyncShellCommand(ShellCommandProperties {
                                command: s!("ls"),
                            }),
                            action: ActionKind::Next,
                            ..Command::default()
                        },
                    ),
                    (
                        command_b_uuid,
                        Command {
                            value: s!("pwd"),
                            kind: CommandKind::SyncShellCommand(ShellCommandProperties {
                                command: s!("pwd"),
                            }),
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
}
