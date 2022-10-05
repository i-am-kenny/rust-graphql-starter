use std::env;

use secrecy::SecretString;

pub struct MyConfig {
    pub addr: String,

    pub port: String,

    pub service_name: String,

    pub is_introspection_disabled: bool,

    pub apollo: Option<ApolloConfig>,
}

pub struct ApolloConfig {
    pub key: SecretString,

    pub graph_ref: String,

    pub graph_version: String,
}

impl MyConfig {
    pub fn from_env() -> Self {
        // dotenv makes .env file content available via env::var()
        dotenv::dotenv().ok();

        Self {
            addr: env::var("ADDR").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT").unwrap_or_else(|_| "8000".into()),
            service_name: "Rust GraphQL".into(),
            is_introspection_disabled: false,
            apollo: None,
        }
    }
}
