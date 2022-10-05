use std::sync::Arc;

use fluent_templates::Loader;
use sqlx::{sqlite::SqliteRow, Row};

use crate::{
    db::Db,
    i18n::{LANGUAGE_MAP, LOCALES, US_ENGLISH},
};

/// Alias type for RequestContextInner, Arc allows the struct to be easily shared between threads.
pub type RequestContext = Arc<RequestContextInner>;

/// Data that would be useful during a GraphQL request.
pub struct RequestContextInner {
    pub user_id: String,

    pub user_language: String,

    pub(crate) db: Arc<Db>,
}

impl RequestContextInner {
    pub fn translate(&self, key: &str) -> String {
        LOCALES
            .lookup(
                LANGUAGE_MAP.get(&self.user_language).unwrap_or(&US_ENGLISH),
                key,
            )
            .unwrap_or_else(|| panic!("i18n key '{key}' missing"))
    }

    pub async fn get_todos(&self) -> Result<Vec<TodoModel>, ()> {
        let res = sqlx::query("SELECT id, status, title, description FROM todo;")
            .fetch_all(&self.db.pool)
            .await
            .map_err(|_| ())?
            .into_iter()
            .map(|row| row.into())
            .collect();

        Ok(res)
    }

    pub async fn get_todo(&self, id: &String) -> Result<TodoModel, ()> {
        sqlx::query("SELECT id, status, title, description FROM todo WHERE id = ?1;")
            .bind(&id)
            .fetch_one(&self.db.pool)
            .await
            .map(|row| row.into())
            .map_err(|_| ())
    }

    pub async fn save_new_todo(&self, model: &TodoModel) -> Result<(), ()> {
        let rows = sqlx::query("INSERT INTO todo VALUES (?1, ?2, ?3, ?4);")
            .bind(&model.id)
            .bind(&model.status)
            .bind(&model.title)
            .bind(&model.description)
            .execute(&self.db.pool)
            .await
            .map_err(|_| ())?
            .rows_affected();

        if rows == 1 {
            return Ok(());
        }

        Err(())
    }

    pub async fn update_todo_status(
        &self,
        id: &String,
        new_status: &String,
    ) -> Result<TodoModel, ()> {
        let rows = sqlx::query("UPDATE todo SET status = ?2 WHERE id = ?1;")
            .bind(id)
            .bind(&new_status)
            .execute(&self.db.pool)
            .await
            .map_err(|_| ())?
            .rows_affected();

        if rows == 1 {
            return self.get_todo(id).await;
        }

        Err(())
    }
}

#[derive(Debug)]
pub struct TodoModel {
    pub id: String,

    pub status: String,

    pub title: String,

    pub description: Option<String>,
}

impl From<SqliteRow> for TodoModel {
    fn from(row: SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            status: row.get("status"),
            title: row.get("title"),
            description: row.get("description"),
        }
    }
}
