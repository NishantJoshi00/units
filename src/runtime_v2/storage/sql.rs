pub struct SqliteStorage {
    pool: sqlx::SqlitePool,
}

impl SqliteStorage {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }
}
