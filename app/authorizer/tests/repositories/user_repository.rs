use authorizer::repositories::user_repository;
use authorizer::models::user;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;

#[tokio::test]
async fn test_insert_and_find_user() {
    // Load DATABASE_URL from `.test.env`
    dotenvy::from_filename(".test.env").ok();
    let db_url = std::env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");
    let db: DbConn = Database::connect(&db_url).await.expect("failed to connect");

    user::Entity::delete_many().exec(&db).await.expect("cleanup failed");

    let test_email = format!("test-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &test_email)
                .await
                .expect("failed to insert user");

    assert_eq!(user.email, test_email);

    let found = user_repository::find_by_email(&db, &test_email)
                    .await
                    .expect("failed to query user");
    
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test]
async fn test_email_uniqueness_violation() {
    dotenvy::from_filename(".test.env").ok();
    let db_url = std::env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");
    let db: DbConn = Database::connect(&db_url).await.expect("failed to connect");

    user::Entity::delete_many().exec(&db).await.expect("cleanup failed");

    let test_email = format!("unique-{}@example.com", Uuid::new_v4());

    // Insert the first user (should succeed)
    let first = user_repository::insert_new_user(&db, &test_email)
                    .await
                    .expect("first insert failed");
    
    assert_eq!(first.email, test_email);

    // Try to insert a second user with the same email (should fail)
    let second = user_repository::insert_new_user(&db, &test_email).await;

    assert!(
        second.is_err(),
        "Expected error due to unique constraint, but insert succeeded"
    );
}

#[tokio::test]
async fn test_email_max_length_constraint() {
    dotenvy::from_filename(".test.env").ok();

    let db_url = std::env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");
    let db: DbConn = Database::connect(&db_url).await.expect("failed to connect");

    user::Entity::delete_many().exec(&db).await.expect("cleanup failed");
    
    // Over 254 characters
    let long_email = format!("long-{}-{}@example.com", Uuid::new_v4(), "a".repeat(255));

    let result = user_repository::insert_new_user(&db, &long_email).await;

    assert!(
        result.is_err(),
        "Expected error due to email length exceeding 254 characters, but insert succeeded"
    );
}