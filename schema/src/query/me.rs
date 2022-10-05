use async_graphql::{Context, Object};
use context::RequestContext;

use crate::types::User;

#[derive(Default)]
pub struct MeQuery;

#[Object]
impl MeQuery {
    pub async fn me(&self, ctx: &Context<'_>) -> User {
        let request_context = ctx.data_unchecked::<RequestContext>();

        User {
            id: request_context.user_id.clone().into(),
            name: "Boba Fett".into(),
            avatar: Some("foo".into()),
        }
    }
}
