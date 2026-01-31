use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;
use std::time::Duration;

pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;
        
        info!("Database connection established");
        
        // Run migrations
        info!("Running database migrations...");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        info!("Database migrations completed");
        
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
