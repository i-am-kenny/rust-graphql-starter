use sqlx::{Pool, Sqlite, SqlitePool};

pub struct Db {
    pub pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("failed creating to SQLite pool");

        let mut conn = pool.acquire().await.expect("failed connecting to SQLite");

        sqlx::query(
            r#"
            CREATE TABLE todo (
                id TEXT NOT NULL,
                status TEXT NOT NULL,
                title TEXT,
                description TEXT
            );
            "#,
        )
        .execute(&mut conn)
        .await
        .expect("failed creating todo table");

        sqlx::query(
            r#"
            INSERT INTO todo VALUES ('MwQM35', 'NOT_STARTED', 'Do thing', NULL);
            INSERT INTO todo VALUES ('lhVa3R', 'IN_PROGRESS', 'Another thing', NULL);
            INSERT INTO todo VALUES ('48cvgN', 'DONE', 'One more thing', NULL);
            "#,
        )
        .execute(&mut conn)
        .await
        .expect("failed inserting seed data");

        Self { pool }
    }
}
