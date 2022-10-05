use async_graphql::MergedObject;

mod hello_world;
mod me;
mod todos;

#[derive(MergedObject, Default)]
pub struct QueryRoot(hello_world::HelloWorldQuery, me::MeQuery, todos::TodosQuery);
