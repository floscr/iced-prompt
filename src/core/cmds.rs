use std::collections::HashMap;
use uuid::Uuid;

pub const SIMPLE_CMD_HEIGHT: f32 = 28.;

#[derive(Debug, Clone)]
pub enum Cmd {
    Simple(Base),
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Default, Clone)]
pub struct Cmds {
    pub cmds: HashMap<Uuid, Cmd>,
    pub order: Vec<Uuid>,
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

#[derive(Debug, Default, Clone)]
pub struct FilteredCmds {
    pub cmds: Vec<Uuid>,
}
