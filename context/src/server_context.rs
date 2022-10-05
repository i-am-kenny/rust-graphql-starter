use std::sync::Arc;

use crate::{db::Db, RequestContext, RequestContextInner};

/// Alias type for ServerContextInner, Arc allows the struct to be easily shared between threads.
pub type ServerContext = Arc<ServerContextInner>;

/// "Singleton"-scoped context for the entire service.
/// Created on started, great place to put shared database connections.
pub struct ServerContextInner {
    db: Arc<Db>,
}

impl ServerContextInner {
    pub async fn new() -> ServerContext {
        let db = Db::new().await;

        Arc::new(ServerContextInner { db: db.into() })
    }

    pub fn new_request_context(&self, user_id: String, user_language: String) -> RequestContext {
        Arc::new(RequestContextInner {
            user_id,
            user_language,
            db: self.db.clone(),
        })
    }
}
