use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    SqlitePool::connect("sqlite:urls.db")
        .await
        .expect("Failed to connect to DB")
}
