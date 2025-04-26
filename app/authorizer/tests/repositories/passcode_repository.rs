use authorizer::config::AppConfig;
use authorizer::models::{passcode, user};
use authorizer::repositories::{passcode_repository, user_repository};
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DbConn, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use serial_test::serial;
use uuid::Uuid;

fn new_test_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

async fn setup_db() -> (DbConn, AppConfig) {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db = Database::connect(&config.database_test_url)
        .await
        .expect("Failed to connect to DB");

    // ⚠️ Order matters: delete child table before parent
    passcode::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    (db, config)
}

#[tokio::test]
#[serial]
async fn test_generate_and_insert_creates_valid_passcode() {
    let (db, config) = setup_db().await;

    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();
    let pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    assert_eq!(pass.user_id, user.id);
    assert_eq!(pass.used, false);
    assert_eq!(pass.attempt_count, 0);
    assert_eq!(pass.resend_count, 0);
    assert_eq!(pass.code.len() as u32, config.passcode_len);
    assert!(pass.expired_at > Utc::now());
}

#[tokio::test]
#[serial]
async fn test_find_active_returns_passcode() {
    let (db, config) = setup_db().await;

    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();
    let created = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    let found = passcode_repository::find_active_by_user_id(&db, user.id)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, created.id);
}

#[tokio::test]
#[serial]
async fn test_find_active_returns_none_if_used() {
    let (db, config) = setup_db().await;

    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();
    let mut active = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap()
        .into_active_model();

    active.used = Set(true);
    active.update(&db).await.unwrap();

    let found = passcode_repository::find_active_by_user_id(&db, user.id)
        .await
        .unwrap();

    assert!(found.is_none());
}

#[tokio::test]
#[serial]
async fn test_find_active_returns_none_if_expired() {
    let (db, _config) = setup_db().await;

    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();

    let expired = passcode::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        code: Set("0001".into()),
        attempt_count: Set(0),
        resend_count: Set(0),
        used: Set(false),
        expired_at: Set(Utc::now() - Duration::seconds(10)),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };
    expired.insert(&db).await.unwrap();

    let found = passcode_repository::find_active_by_user_id(&db, user.id)
        .await
        .unwrap();

    assert!(found.is_none());
}

#[tokio::test]
#[serial]
async fn test_find_active_returns_latest_if_multiple_valid() {
    let (db, config) = setup_db().await;

    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();

    let _old = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let latest = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    let found = passcode_repository::find_active_by_user_id(&db, user.id)
        .await
        .unwrap();
    assert_eq!(found.unwrap().id, latest.id);
}

#[tokio::test]
#[serial]
async fn test_find_matching_active_passcode_success() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    let found = passcode_repository::find_matching_active_passcode(&db, user.id, &pass.code)
        .await
        .unwrap();

    assert!(found.is_some());
    assert_eq!(found.unwrap().id, pass.id);
}

#[tokio::test]
#[serial]
async fn test_find_matching_active_passcode_returns_none_if_used() {
    let (db, config) = setup_db().await;

    let email = format!("verrify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let mut pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap()
        .into_active_model();

    let code = pass.code.as_ref().clone();

    pass.used = Set(true);
    pass.update(&db).await.unwrap();

    let found = passcode_repository::find_matching_active_passcode(&db, user.id, &code)
        .await
        .unwrap();

    assert!(found.is_none());
}

#[tokio::test]
#[serial]
async fn test_increment_attempt_count_increments_properly() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    passcode_repository::increment_attempt_count(&db, pass.id)
        .await
        .unwrap();

    let updated = passcode::Entity::find_by_id(pass.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.attempt_count, 1);
}

#[tokio::test]
#[serial]
async fn test_mark_all_active_codes_used_sets_used_flag() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    for _ in 0..3 {
        passcode_repository::generate_and_insert(&db, &config, &user)
            .await
            .unwrap();
    }

    passcode_repository::mark_all_active_codes_used(&db, user.id)
        .await
        .unwrap();

    let all = passcode::Entity::find()
        .filter(passcode::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .unwrap();

    assert!(all.iter().all(|p| p.used));
}

#[tokio::test]
#[serial]
async fn test_increment_resend_count_increments_properly() {
    let (db, config) = setup_db().await;

    let email = format!("resend-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    passcode_repository::increment_resend_count(&db, pass.id)
        .await
        .unwrap();

    let updated = passcode::Entity::find_by_id(pass.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated.resend_count, 1);
}
