use async_graphql::{Context, Object};
use context::RequestContext;

#[derive(Default)]
pub struct HelloWorldQuery;

#[Object]
impl HelloWorldQuery {
    pub async fn hello_world(&self, ctx: &Context<'_>) -> String {
        let request_context = ctx.data_unchecked::<RequestContext>();

        request_context.translate("hello_world")
    }
}
