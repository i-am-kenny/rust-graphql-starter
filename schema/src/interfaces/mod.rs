use async_graphql::{Interface, ID};

use crate::types::{Todo, User};

#[derive(Interface)]
#[graphql(field(name = "id", type = "&ID"))]
pub enum Node {
    User(User),
    Todo(Todo),
}
