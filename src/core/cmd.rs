use uuid::Uuid;

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
}
