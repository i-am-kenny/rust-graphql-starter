mod interfaces;
mod mutation;
mod query;
mod scalars;
mod types;

use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use mutation::MutationRoot;
use query::QueryRoot;

/// The project schema.
pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Builds the schema while "hiding" all the interior QueryRoot and MutationRoot structs.
pub fn build_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
}
