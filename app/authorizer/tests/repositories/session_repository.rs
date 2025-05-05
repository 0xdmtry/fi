use authorizer::config::AppConfig;
use authorizer::models::{session, user};
use authorizer::repositories::{session_repository, user_repository};
use chrono::{Duration, Utc};
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;

#[tokio::test]
async fn test_create_and_find_active_session() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    // Clean up sessions and users
    session::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");
    user::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let email = format!("user-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email)
        .await
        .expect("failed to insert user");

    let expires_at = Utc::now() + Duration::days(1);
    let session = session_repository::create_session(
        &db,
        user.id,
        "Mozilla/5.0 (X11; Linux x86_64)",
        "127.0.0.1",
        expires_at,
    )
    .await
    .expect("failed to create session");

    let found = session_repository::find_active_session(&db, session.id)
        .await
        .expect("failed to find session");

    assert!(found.is_some());
    assert_eq!(found.unwrap().id, session.id);
}

#[tokio::test]
async fn test_revoke_session_and_fail_find() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    session::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");
    user::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let email = format!("user-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email)
        .await
        .expect("failed to insert user");

    let expires_at = Utc::now() + Duration::days(1);
    let session = session_repository::create_session(
        &db,
        user.id,
        "Mozilla/5.0 (X11; Linux x86_64)",
        "127.0.0.1",
        expires_at,
    )
    .await
    .expect("failed to create session");

    session_repository::revoke_session(&db, session.id)
        .await
        .expect("failed to revoke session");

    let found = session_repository::find_active_session(&db, session.id)
        .await
        .expect("find should succeed but return None");

    assert!(found.is_none());
}

#[tokio::test]
async fn test_revoke_all_user_sessions() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    session::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");
    user::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let email = format!("user-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email)
        .await
        .expect("failed to insert user");

    let expires_at = Utc::now() + Duration::days(1);

    // Create multiple sessions
    for _ in 0..3 {
        session_repository::create_session(&db, user.id, "TestAgent/1.0", "127.0.0.1", expires_at)
            .await
            .expect("failed to create session");
    }

    session_repository::revoke_all_user_sessions(&db, user.id)
        .await
        .expect("failed to revoke all");

    // Ensure no active sessions remain
    let all = session::Entity::find()
        .filter(session::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .expect("failed to query sessions");

    assert!(!all.is_empty());
    for s in all {
        assert!(s.revoked_at.is_some(), "Expected session to be revoked");
    }
}
