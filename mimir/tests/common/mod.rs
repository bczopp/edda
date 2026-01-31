// Common test utilities and helpers

use std::time::Duration;
use sqlx::{PgPool, Postgres, Pool};
use testcontainers::{clients, Container, images::postgres::Postgres as PostgresImage};

pub struct TestDatabase {
    pub pool: PgPool,
    _container: Container<'_, PostgresImage>,
}

impl TestDatabase {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let docker = clients::Cli::default();
        let postgres_image = PostgresImage::default();
        let container = docker.run(postgres_image);
        
        let port = container.get_host_port_ipv4(5432);
        let database_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
        
        // Wait for database to be ready
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        let pool = PgPool::connect(&database_url).await?;
        
        // Run migrations
        sqlx::migrate!("../migrations").run(&pool).await?;
        
        Ok(Self {
            pool,
            _container: container,
        })
    }
}

pub fn setup_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}
