pub struct DatabaseManager {
    pub pool: sqlx::Pool<sqlx::Sqlite>
}

#[async_trait::async_trait]
pub trait WriteDBItem<T> {
   async fn write(&self, item: &T);
}