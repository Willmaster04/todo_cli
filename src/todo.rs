use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum TodoStatus {
    Done,
    NotDone
}

#[derive(Debug)]
pub(crate) struct Todo {
    pub(crate) name: String,
    pub(crate) status: TodoStatus
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.status == TodoStatus::Done {
            write!(f, "■ {}", self.name)
        } else {
            write!(f, "□ {}", self.name)
        }
    }
}

impl Todo {
    pub(crate) fn new(name: &str) -> Todo {
        Todo {
            name: name.trim().to_owned(),
            status: TodoStatus::NotDone
        }
    }

    pub(crate) fn new_from_file(name: &str, status: TodoStatus) -> Todo {
        Todo {
            name: name.trim().to_owned(),
            status: status
        }
    }
}