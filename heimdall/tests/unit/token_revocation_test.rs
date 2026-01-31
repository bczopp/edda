//! Tests for TokenRevocationManager (Phase 6.4.1): immediate revoke, revocation list, cache invalidation.

use heimdall::token::TokenRevocationManager;
use heimdall::utils::token_repository::TokenRepository;
use std::sync::Arc;
use crate::common::{TestDatabase, create_test_device};
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_revoke_token_succeeds() {
    let test_db = TestDatabase::new().await.unwrap();
    let repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
    let mgr = TokenRevocationManager::new(repo.clone(), None);

    let token_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4();
    let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
    repo.create(
        &token_id,
        device.id,
        user_id,
        "heimdall",
        "token-data",
        Utc::now() + chrono::Duration::hours(1),
    )
    .await
    .unwrap();

    mgr.revoke(&token_id).await.unwrap();

    let token = repo.get_by_token_id(&token_id).await.unwrap();
    assert!(token.is_revoked);
}

#[tokio::test]
async fn test_revoke_nonexistent_token_returns_error() {
    let test_db = TestDatabase::new().await.unwrap();
    let repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
    let mgr = TokenRevocationManager::new(repo, None);

    let res = mgr.revoke("nonexistent-token-id").await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_is_revoked_returns_true_after_revoke() {
    let test_db = TestDatabase::new().await.unwrap();
    let repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
    let mgr = TokenRevocationManager::new(repo.clone(), None);

    let token_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4();
    let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
    repo.create(
        &token_id,
        device.id,
        user_id,
        "session",
        "data",
        Utc::now() + chrono::Duration::hours(1),
    )
    .await
    .unwrap();

    assert!(!mgr.is_revoked(&token_id).await.unwrap());
    mgr.revoke(&token_id).await.unwrap();
    assert!(mgr.is_revoked(&token_id).await.unwrap());
}

#[tokio::test]
async fn test_is_revoked_returns_false_for_nonexistent_token() {
    let test_db = TestDatabase::new().await.unwrap();
    let repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
    let mgr = TokenRevocationManager::new(repo, None);

    let revoked = mgr.is_revoked("nonexistent").await.unwrap();
    assert!(!revoked);
}
