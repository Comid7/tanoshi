use sqlx::any::{AnyPool, AnyPoolOptions};

pub async fn establish_connection(database_path: String) -> AnyPool {
    AnyPoolOptions::new()
        .max_connections(5)
        .connect(&database_path)
        .await
        .unwrap()
}
