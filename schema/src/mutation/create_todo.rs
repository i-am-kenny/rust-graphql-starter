use async_graphql::{Context, InputObject, Object, SimpleObject, Union};
use context::{RequestContext, TodoModel};
use nanoid::nanoid;

use crate::types::{Todo, TodoStatus};

#[derive(Default)]
pub struct CreateTodoMutation;

#[Object]
impl CreateTodoMutation {
    pub async fn create_todo(&self, ctx: &Context<'_>, input: CreateTodoInput) -> CreateTodoResult {
        let request_context = ctx.data_unchecked::<RequestContext>();

        let model = TodoModel {
            id: nanoid!(6),
            status: TodoStatus::NotStarted.into(),
            title: input.title,
            description: input.description,
        };

        match request_context.save_new_todo(&model).await {
            Ok(_) => CreateTodoResult::Success(CreateTodoSuccess { todo: model.into() }),

            Err(_) => CreateTodoResult::Error(CreateTodoError {
                message: "unexpected error".into(),
            }),
        }
    }
}

#[derive(InputObject)]
pub struct CreateTodoInput {
    pub title: String,

    pub description: Option<String>,
}

#[derive(Union)]
pub enum CreateTodoResult {
    Success(CreateTodoSuccess),
    Invalid(CreateTodoInvalid),
    Error(CreateTodoError),
}

#[derive(SimpleObject)]
pub struct CreateTodoSuccess {
    pub todo: Todo,
}

#[derive(SimpleObject)]
pub struct CreateTodoInvalid {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct CreateTodoError {
    pub message: String,
}
