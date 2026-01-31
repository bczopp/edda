// Common test utilities and helpers
//
// Tests run in container where DATABASE_URL is set (docker-compose.test.yml).
// For local runs, set DATABASE_URL to a Postgres URL or run via container.

use sqlx::PgPool;

pub struct TestDatabase {
    pub pool: PgPool,
}

impl TestDatabase {
    /// Connect using DATABASE_URL and run migrations. Required for all tests (run in container).
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            let msg = "DATABASE_URL must be set. Run tests in container: docker compose -f docker-compose.test.yml run --rm heimdall-test";
            Box::<dyn std::error::Error>::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))
        })?;
        let pool = PgPool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(TestDatabase { pool })
    }
}

/// Create a device row so sessions/tokens can reference devices(id). Returns the created Device (use .id for FK).
pub async fn create_test_device(
    pool: &PgPool,
    device_id: &str,
    user_id: uuid::Uuid,
) -> Result<heimdall::utils::models::Device, Box<dyn std::error::Error>> {
    let repo = heimdall::utils::device_repository::DeviceRepository::new(pool.clone());
    repo.create(device_id, user_id, "test-key", None, None)
        .await
        .map_err(Into::into)
}

pub fn setup_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}
