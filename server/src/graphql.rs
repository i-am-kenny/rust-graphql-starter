use async_graphql::extensions::apollo_persisted_queries::{
    ApolloPersistedQueries, LruCacheStorage,
};
use async_graphql_extension_apollo_tracing::{register, ApolloTracing};
use schema::MySchema;
use secrecy::ExposeSecret;

use crate::config::MyConfig;

pub async fn initialize(config: &MyConfig) -> MySchema {
    let apq_cache = LruCacheStorage::new(1_000);
    let mut schema = schema::build_schema().extension(ApolloPersistedQueries::new(apq_cache));

    if config.is_introspection_disabled {
        schema = schema.disable_introspection();
    }

    if let Some(apollo) = &config.apollo {
        schema = schema.extension(ApolloTracing::new(
            apollo.key.expose_secret().into(),
            "".into(),
            apollo.graph_ref.clone(),
            apollo.graph_version.clone(),
            100,
        ));
    }

    let schema = schema.finish();

    if let Some(apollo) = &config.apollo {
        let variant = apollo
            .graph_ref
            .split('@')
            .nth(1)
            .expect("invalid graph_ref");

        register::register(
            apollo.key.expose_secret(),
            &schema,
            "",
            variant,
            &apollo.graph_version,
            "",
        )
        .await
        .expect("failed registering schema with Apollo");
    }

    schema
}
