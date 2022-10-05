use async_graphql::scalar;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url(String);

scalar!(Url);

impl From<&str> for Url {
    fn from(v: &str) -> Self {
        Self(v.to_string())
    }
}
