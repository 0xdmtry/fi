use emailer::config::AppConfig;
use emailer::models::email;
use emailer::models::{EmailType, Provider};
use emailer::repositories::email_repository::SaveEmailArgs;
use emailer::services::Emailer;
use emailer::services::mailhog_service::MailhogEmailer;
use lettre::{Message, SmtpTransport, Transport};
use sea_orm::{ColumnTrait, Database, DbConn, EntityTrait, QueryFilter};
use serial_test::serial;
use uuid::Uuid;

fn test_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

async fn setup_config_and_db() -> (DbConn, AppConfig) {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db = Database::connect(&config.database_test_url).await.unwrap();

    email::Entity::delete_many().exec(&db).await.unwrap();

    (db, config)
}

#[tokio::test]
#[serial]
async fn test_send_passcode_email_succeeds() {
    let emailer = MailhogEmailer::new();
    let (_, config) = setup_config_and_db().await;

    let result = emailer.send_passcode_email(&config, "user@example.com", "1234");

    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_save_email_persists_to_db() {
    let (db, config) = setup_config_and_db().await;
    let emailer = MailhogEmailer::new();

    let args = SaveEmailArgs {
        recipient: test_email(),
        email_type: EmailType::Passcode,
        subject: Some("Subject".into()),
        content: Some("Content".into()),
        template_type: Some("default".into()),
        template_language: Some("en".into()),
        rendered_with: Some("askama".into()),
        provider: Provider::Mailhog,
        status: "sent".into(),
        error_message: None,
        message_id: None,
        retry_count: 0,
        sent_by_fallback: false,
        opened_at: None,
    };

    let _ = emailer.save_email(&config, &db, args.clone());

    // Allow the spawned task to complete
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    let found = email::Entity::find()
        .filter(email::Column::Recipient.eq(args.recipient.clone()))
        .one(&db)
        .await
        .unwrap();

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.recipient, args.recipient);
    assert_eq!(found.status, "sent");
}

#[tokio::test]
#[serial]
async fn test_send_and_save_passcode_email_succeeds() {
    let (db, config) = setup_config_and_db().await;
    let emailer = MailhogEmailer::new();

    let to = test_email();
    let passcode = "5678";

    let result = emailer.send_and_save_passcode_email(&config, &db, &to, passcode);

    assert!(result.is_ok());

    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    let saved = email::Entity::find()
        .filter(email::Column::Recipient.eq(to))
        .one(&db)
        .await
        .unwrap();

    assert!(saved.is_some());
}

// Failing cases

#[tokio::test]
#[serial]
async fn test_send_passcode_email_with_invalid_email_fails() {
    let emailer = MailhogEmailer::new();
    let (_, config) = setup_config_and_db().await;

    let result = emailer.send_passcode_email(&config, "not-an-email", "1234");

    assert!(result.is_err());
    let err = result.err().unwrap().to_string();
    assert!(err.contains("invalid email address syntax"));
}

#[tokio::test]
#[serial]
async fn test_send_passcode_email_with_too_long_email_fails() {
    let emailer = MailhogEmailer::new();
    let (_, config) = setup_config_and_db().await;

    let long_email = format!("{}@example.com", "a".repeat(245));
    let result = emailer.send_passcode_email(&config, &long_email, "1234");

    assert!(result.is_err());
    let err = result.err().unwrap().to_string();
    assert!(err.contains("invalid email address"));
}

#[tokio::test]
#[serial]
async fn test_save_email_with_missing_recipient_logs_error() {
    let (db, config) = setup_config_and_db().await;
    let emailer = MailhogEmailer::new();

    let args = SaveEmailArgs {
        recipient: "".into(), // ⚠️ Invalid
        email_type: EmailType::Passcode,
        subject: None,
        content: None,
        template_type: None,
        template_language: None,
        rendered_with: None,
        provider: Provider::Mailhog,
        status: "sent".into(),
        error_message: None,
        message_id: None,
        retry_count: 0,
        sent_by_fallback: false,
        opened_at: None,
    };

    let result = emailer.save_email(&config, &db, args);
    assert!(result.is_ok()); // still returns Ok (spawned task)

    // Let the DB insert fail in background
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    // Should not have been inserted
    let results = email::Entity::find().all(&db).await.unwrap();
    assert!(results.is_empty());
}

#[tokio::test]
#[serial]
async fn test_send_email_to_unreachable_host_fails() {
    // simulate "unreachable" by changing host/port
    let message = Message::builder()
        .from("noreply@example.com".parse().unwrap())
        .to("user@example.com".parse().unwrap())
        .subject("Failing send")
        .body("test".to_string())
        .expect("failed to build message");

    let result = SmtpTransport::builder_dangerous("127.0.0.1")
        .port(9999)
        .build()
        .send(&message);

    assert!(result.is_err());
}
