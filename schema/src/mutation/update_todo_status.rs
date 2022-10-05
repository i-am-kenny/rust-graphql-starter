use async_graphql::{Context, InputObject, Object, SimpleObject, Union, ID};
use context::RequestContext;

use crate::types::{Todo, TodoStatus};

#[derive(Default)]
pub struct UpdateTodoStatusMutation;

#[Object]
impl UpdateTodoStatusMutation {
    pub async fn update_todo_status(
        &self,
        ctx: &Context<'_>,
        input: UpdateTodoStatusInput,
    ) -> UpdateTodoStatusResult {
        let request_context = ctx.data_unchecked::<RequestContext>();

        let res = request_context
            .update_todo_status(&input.id, &input.status.into())
            .await;

        match res {
            Ok(model) => {
                UpdateTodoStatusResult::Success(UpdateTodoStatusSuccess { todo: model.into() })
            }

            Err(_) => UpdateTodoStatusResult::Error(UpdateTodoStatusError {
                message: "error updating status".into(),
            }),
        }
    }
}

#[derive(InputObject)]
pub struct UpdateTodoStatusInput {
    pub id: ID,

    pub status: TodoStatus,
}

#[derive(Union)]
pub enum UpdateTodoStatusResult {
    Success(UpdateTodoStatusSuccess),
    Error(UpdateTodoStatusError),
}

#[derive(SimpleObject)]
pub struct UpdateTodoStatusSuccess {
    pub todo: Todo,
}

#[derive(SimpleObject)]
pub struct UpdateTodoStatusError {
    pub message: String,
}
