use std::collections::HashMap;
use uuid::Uuid;

use crate::utils::list::SinglyLinkedList;

pub const SIMPLE_CMD_HEIGHT: f32 = 28.;

// Command ---------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Cmd {
    Simple(Base),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Base {
    pub id: Uuid,
    pub value: String,
}

impl Cmd {
    pub fn new(value: String) -> Cmd {
        Cmd::Simple(Base {
            id: Uuid::new_v4(),
            value,
        })
    }

    pub fn value(cmd: &Cmd) -> &String {
        match cmd {
            Cmd::Simple(base) => &base.value,
        }
    }

    pub fn uuid(cmd: &Cmd) -> &Uuid {
        match cmd {
            Cmd::Simple(base) => &base.id,
        }
    }

    pub fn height(cmd: &Cmd) -> f32 {
        match cmd {
            Cmd::Simple(_) => SIMPLE_CMD_HEIGHT + 1.,
        }
    }
}

// Commands --------------------------------------------------------------------

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Cmds {
    pub cmds: HashMap<Uuid, Cmd>,
    pub order: Vec<Uuid>,
}

#[derive(Debug, Default, Clone)]
pub struct FilteredCmds {
    pub cmds: Vec<Uuid>,
}

impl Cmds {
    pub fn from_string(data: String) -> Cmds {
        let mut cmds = HashMap::new();
        let mut order = Vec::new();

        for line in data.split('\n') {
            let id = Uuid::new_v4();
            let cmd = Cmd::Simple(Base {
                id,
                value: line.to_string(),
            });
            cmds.insert(id, cmd);
            order.push(id);
        }

        Cmds { cmds, order }
    }

    pub fn get_by_index(&self, index: usize) -> Option<Cmd> {
        self.order
            .get(index)
            .and_then(|id| self.cmds.get(id).cloned())
    }

    pub fn map<F, T>(&self, mut f: F) -> Vec<T>
    where
        F: FnMut(usize, &Uuid, &Cmd) -> T,
    {
        self.order
            .iter()
            .enumerate()
            .filter_map(|(index, id)| self.cmds.get(id).map(|cmd| f(index, id, cmd)))
            .collect()
    }

    pub fn scroll_offset_at_index(self, index: usize) -> f32 {
        let ids = &self.order[..index];
        let mut offset = 0.;
        for id in ids {
            offset += Cmd::height(&self.cmds[id])
        }
        offset
    }

    pub fn with_filtered_order(self, filtered_cmds: &FilteredCmds) -> Cmds {
        Cmds {
            cmds: self.cmds,
            order: filtered_cmds.cmds.clone(),
        }
    }

    pub fn filter_by_value(&self, substring: &str) -> FilteredCmds {
        let filtered_order: Vec<Uuid> = self
            .order
            .iter()
            .filter(|&id| {
                let cmd = self.cmds.get(id).expect("Order contains invalid id");
                let value = Cmd::value(cmd);
                value.to_lowercase().contains(&substring.to_lowercase())
            })
            .cloned()
            .collect();

        FilteredCmds {
            cmds: filtered_order,
        }
    }
}

// History ---------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct History {
    pub history: SinglyLinkedList<Cmds>,
}

impl History {
    pub fn push(self, cmds: Cmds) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.push(cmds);
        History { history: cmds_list }
    }

    pub fn pop(self) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.pop();
        History { history: cmds_list }
    }

    pub fn head(self) -> Option<Cmds> {
        let mut cmds_list = self.history.clone();
        cmds_list.pop()
    }

    pub fn len(self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Cmds, History};

    fn it_works() {
        let history_with_two_items = History::default()
            .push(Cmds::default())
            .push(Cmds::default())
            .pop();

        assert_eq!(history_with_two_items.len(), 2);
    }

    fn test_head() {
        assert_eq!(History::default().head(), None);
        assert_eq!(
            History::default().push(Cmds::default()).head(),
            Some(Cmds::default())
        );
    }
}
