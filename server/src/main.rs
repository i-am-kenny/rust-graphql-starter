#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_extension_apollo_tracing::ApolloTracingDataExt;
use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};

use context::ServerContext;
use schema::MySchema;
use tracing::Instrument;

mod config;
mod graphql;
mod telemetry;

#[tokio::main]
async fn main() {
    let config = config::MyConfig::from_env();

    telemetry::initialize(&config);

    // initialize the GraphQL schema and Apollo extensions
    let schema = graphql::initialize(&config).await;

    // initialize DB connections, etc..
    let server_context = context::ServerContextInner::new().await;

    let routes = Router::new().fallback(
        get(playground_handler)
            .post(graphql_handler)
            .layer(Extension(schema))
            .layer(Extension(server_context)),
    );

    let bind_addr = format!("{}:{}", config.addr, config.port)
        .parse::<SocketAddr>()
        .expect("bind_addr was malformed");

    println!("🚀 Rust GraphQL is listening at {bind_addr}");

    axum::Server::bind(&bind_addr)
        .serve(routes.into_make_service())
        .await
        .expect("axum server failed to start");
}

lazy_static! {
    static ref EMPTY_OPERATION_NAME: String = "".into();
}

async fn playground_handler() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").with_setting("schema.polling.enable", false),
    ))
}

async fn graphql_handler(
    schema: Extension<MySchema>,
    server_context: Extension<ServerContext>,
    Json(req): Json<async_graphql::Request>,
    headers: HeaderMap,
) -> Json<async_graphql::Response> {
    // TODO: parse / validate authorization headers
    let user_id = String::from("example_user_id_123");
    let user_language = String::from("en-US");

    // Personally, I'd error if the operation name is missing or empty.
    let operation_name = req.operation_name.as_ref().unwrap_or(&EMPTY_OPERATION_NAME);

    let client_name = headers
        .get("apollographql-client-name")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default();

    let client_version = headers
        .get("apollographql-client-version")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default();

    // Create a logging Span with all the request metadata
    let span = tracing::info_span!(
        "graphql_operation",
        user_id = &user_id,
        operation_name = &operation_name,
        client_name = &client_name,
        client_version = &client_version
    );

    // Create a scoped request_context using the user info
    let request_context = server_context.new_request_context(user_id, user_language);

    let tracing_data = ApolloTracingDataExt {
        client_name: Some(client_name.into()),
        client_version: Some(client_version.into()),
        status_code: Some(200),
        ..Default::default()
    };

    schema
        .execute(req.data(tracing_data).data(request_context))
        .instrument(span)
        .await
        .into()
}
