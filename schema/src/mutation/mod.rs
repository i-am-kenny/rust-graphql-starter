use async_graphql::MergedObject;

mod create_todo;
mod update_todo_status;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    create_todo::CreateTodoMutation,
    update_todo_status::UpdateTodoStatusMutation,
);
