use crate::core::commands::Command;
use crate::utils::list::SinglyLinkedList;

#[derive(Debug, Default, Clone)]
pub struct History {
    pub history: SinglyLinkedList<Command>,
}

#[allow(dead_code)]
impl History {
    pub fn push(self, cmds: Command) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.push(cmds);
        History { history: cmds_list }
    }

    pub fn pop(self) -> History {
        let mut cmds_list = self.history.clone();
        cmds_list.pop();
        History { history: cmds_list }
    }

    pub fn pop_with_minimum(self) -> History {
        if self.history.len() != 1 {
            let mut cmds_list = self.history.clone();
            cmds_list.pop();
            History { history: cmds_list }
        } else {
            self
        }
    }

    pub fn head(&self) -> Option<Command> {
        let mut cmds_list = self.history.clone();
        cmds_list.pop()
    }

    pub fn split(self) -> Option<(Command, SinglyLinkedList<Command>)> {
        let mut cmds_list = self.history.clone();
        cmds_list.pop().map(|cmds| (cmds, cmds_list.clone()))
    }

    pub fn len(self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, History};

    fn it_works() {
        let history_with_two_items = History::default()
            .push(Command::default())
            .push(Command::default())
            .pop();

        assert_eq!(history_with_two_items.len(), 2);
    }

    fn test_head() {
        assert_eq!(History::default().head(), None);
        assert_eq!(
            History::default().push(Command::default()).head(),
            Some(Command::default())
        );
    }

    fn test_split() {
        let (head, tail) = History::default()
            .push(Command::default())
            .push(Command::default())
            .split()
            .unwrap();
        assert_eq!(head, Command::default());
        assert_eq!(tail.len(), 1);
    }
}
