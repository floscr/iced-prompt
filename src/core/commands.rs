use std::collections::HashMap;

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;
use std::fmt;

use uuid::Uuid;

// Commands Deserializer -------------------------------------------------------

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

    #[test]
    fn deserializes_nested_command() {
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
    }

    ]
}"#;

        let v: Command = serde_json::from_str(data).unwrap();

        println!("{:#?}", v);

        assert_eq!(v.value, "Commands");
        assert_eq!(v.items.order.len(), 1);
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
    }
}
