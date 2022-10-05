use async_graphql::{Enum, SimpleObject, ID};
use context::TodoModel;

#[derive(SimpleObject)]
pub struct Todo {
    pub id: ID,

    pub status: TodoStatus,

    pub title: String,

    pub description: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TodoStatus {
    NotStarted,
    InProgress,
    Done,
}

impl From<TodoModel> for Todo {
    fn from(t: TodoModel) -> Self {
        Todo {
            id: t.id.into(),
            status: t.status.into(),
            title: t.title,
            description: t.description,
        }
    }
}

impl From<String> for TodoStatus {
    fn from(v: String) -> Self {
        match v.as_ref() {
            "NOT_STARTED" => Self::NotStarted,
            "IN_PROGRESS" => Self::InProgress,
            "DONE" => Self::Done,
            _ => unreachable!(),
        }
    }
}

impl From<TodoStatus> for String {
    fn from(v: TodoStatus) -> Self {
        match v {
            TodoStatus::NotStarted => "NOT_STARTED".into(),
            TodoStatus::InProgress => "IN_PROGRESS".into(),
            TodoStatus::Done => "DONE".into(),
        }
    }
}
