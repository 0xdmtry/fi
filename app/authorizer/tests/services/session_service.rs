use authorizer::config::AppConfig;
use authorizer::models::{user, session};
use authorizer::repositories::user_repository;
use authorizer::services::session_service;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;
use chrono::{Utc, Duration};

#[tokio::test]
async fn test_create_and_refresh_session() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url).await.unwrap();

    session::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    let email = format!("session-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let (access_token, refresh_token) = session_service::create_session(
        &db,
        &config,
        &user,
        "TestAgent/1.0",
        "127.0.0.1",
    )
    .await
    .unwrap();

    assert!(!access_token.is_empty(), "access_token should not be empty");

    let new_token = session_service::validate_and_refresh(&db, &config, refresh_token)
        .await
        .expect("refresh should succeed");

    assert!(!new_token.is_empty(), "new token should not be empty");
}

#[tokio::test]
async fn test_refresh_invalid_session_should_fail() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url).await.unwrap();

    session::Entity::delete_many().exec(&db).await.unwrap();

    let fake_session_id = Uuid::new_v4();
    let result = session_service::validate_and_refresh(&db, &config, fake_session_id).await;

    assert!(result.is_err(), "expected error from non-existent session");
}

#[tokio::test]
async fn test_revoke_session_should_block_refresh() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url).await.unwrap();

    session::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    let email = format!("revoke-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let (_token, refresh_token) = session_service::create_session(
        &db,
        &config,
        &user,
        "Agent/2.0",
        "10.0.0.2",
    )
    .await
    .unwrap();

    session_service::revoke_session(&db, refresh_token)
        .await
        .expect("revoke failed");

    let result = session_service::validate_and_refresh(&db, &config, refresh_token).await;
    assert!(result.is_err(), "refresh should fail after revocation");
}

#[tokio::test]
async fn test_revoke_all_sessions_for_user() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url).await.unwrap();

    session::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    let email = format!("all-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    for _ in 0..3 {
        session_service::create_session(
            &db,
            &config,
            &user,
            "AgentX",
            "192.168.1.1",
        )
        .await
        .unwrap();
    }

    session_service::revoke_all_sessions(&db, user.id)
        .await
        .expect("revoke all failed");

    let sessions = session::Entity::find()
        .filter(session::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .unwrap();

    for s in sessions {
        assert!(s.revoked_at.is_some(), "Expected all sessions to be revoked");
    }
}
