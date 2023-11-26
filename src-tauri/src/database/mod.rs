pub struct DatabaseManager {
    pub pool: sqlx::Pool<sqlx::Sqlite>
}