use async_graphql::{Context, Object};
use context::RequestContext;

use crate::types::Todo;

#[derive(Default)]
pub struct TodosQuery;

#[Object]
impl TodosQuery {
    pub async fn todos(&self, ctx: &Context<'_>) -> Vec<Todo> {
        let request_context = ctx.data_unchecked::<RequestContext>();

        request_context
            .get_todos()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(Todo::from)
            .collect()
    }
}
