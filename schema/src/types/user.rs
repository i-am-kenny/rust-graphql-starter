use async_graphql::{SimpleObject, ID};

use crate::scalars::Url;

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,

    pub name: String,

    pub avatar: Option<Url>,
}
