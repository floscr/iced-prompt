use std::collections::HashMap;
use uuid::Uuid;

use crate::utils::list::SinglyLinkedList;

pub const SIMPLE_CMD_HEIGHT: f32 = 28.;

// Command ---------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Simple(Base),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Base {
    pub id: Uuid,
    pub value: String,
}

impl Item {
    #[allow(dead_code)]
    pub fn new(value: String) -> Item {
        Item::Simple(Base {
            id: Uuid::new_v4(),
            value,
        })
    }

    pub fn value(cmd: &Item) -> &String {
        match cmd {
            Item::Simple(base) => &base.value,
        }
    }

    pub fn uuid(cmd: &Item) -> &Uuid {
        match cmd {
            Item::Simple(base) => &base.id,
        }
    }

    pub fn height(cmd: &Item) -> f32 {
        match cmd {
            Item::Simple(_) => SIMPLE_CMD_HEIGHT + 1.,
        }
    }
}

// Commands --------------------------------------------------------------------

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ModeKind {
    #[default]
    Default,
    SyncShellCommand(ShellCommandProperties),
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShellCommandProperties {
    command: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mode {
    pub kind: ModeKind,
    pub items: HashMap<Uuid, Item>,
    pub order: Vec<Uuid>,
}

#[derive(Debug, Default, Clone)]
pub struct FilteredItems {
    pub items: Vec<Uuid>,
}

impl Mode {
    pub fn from_string(data: String) -> Mode {
        let mut cmds = HashMap::new();
        let mut order = Vec::new();

        for line in data.split('\n') {
            let id = Uuid::new_v4();
            let cmd = Item::Simple(Base {
                id,
                value: line.to_string(),
            });
            cmds.insert(id, cmd);
            order.push(id);
        }

        Mode {
            kind: ModeKind::Default,
            items: cmds,
            order,
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<Item> {
        self.order
            .get(index)
            .and_then(|id| self.items.get(id).cloned())
    }

    pub fn map<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(usize, &Uuid, &Item) -> T,
    {
        self.order
            .iter()
            .enumerate()
            .filter_map(|(index, id)| self.items.get(id).map(|cmd| f(index, id, cmd)))
            .collect()
    }

    pub fn scroll_offset_at_index(self, index: usize) -> f32 {
        let ids = &self.order[..index];
        let mut offset = 0.;
        for id in ids {
            offset += Item::height(&self.items[id])
        }
        offset
    }

    pub fn with_filtered_order(self, filtered_cmds: &FilteredItems) -> Mode {
        Mode {
            items: self.items,
            order: filtered_cmds.items.clone(),
            ..self
        }
    }

    pub fn filter_by_value(&self, substring: &str) -> FilteredItems {
        let filtered_order: Vec<Uuid> = self
            .order
            .iter()
            .filter(|&id| {
                let cmd = self.items.get(id).expect("Order contains invalid id");
                let value = Item::value(cmd);
                value.to_lowercase().contains(&substring.to_lowercase())
            })
            .cloned()
            .collect();

        FilteredItems {
            items: filtered_order,
        }
    }
}

// History ---------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct History {
    pub history: SinglyLinkedList<Mode>,
}

#[allow(dead_code)]
impl History {
    pub fn push(self, cmds: Mode) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.push(cmds);
        History { history: cmds_list }
    }

    pub fn pop(self) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.pop();
        History { history: cmds_list }
    }

    pub fn head(self) -> Option<Mode> {
        let mut cmds_list = self.history.clone();
        cmds_list.pop()
    }

    pub fn split(self) -> Option<(Mode, SinglyLinkedList<Mode>)> {
        let mut cmds_list = self.history.clone();
        cmds_list.pop().map(|cmds| (cmds, cmds_list.clone()))
    }

    pub fn len(self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{History, Mode};

    fn it_works() {
        let history_with_two_items = History::default()
            .push(Mode::default())
            .push(Mode::default())
            .pop();

        assert_eq!(history_with_two_items.len(), 2);
    }

    fn test_head() {
        assert_eq!(History::default().head(), None);
        assert_eq!(
            History::default().push(Mode::default()).head(),
            Some(Mode::default())
        );
    }

    fn test_split() {
        let (head, tail) = History::default()
            .push(Mode::default())
            .push(Mode::default())
            .split()
            .unwrap();
        assert_eq!(head, Mode::default());
        assert_eq!(tail.len(), 1);
    }
}
