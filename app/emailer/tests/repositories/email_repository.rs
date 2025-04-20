use emailer::config::AppConfig;
use emailer::models::{email, EmailType, Provider};
use emailer::repositories::email_repository::{save_sent_email, SaveEmailArgs};
use chrono::Utc;
use sea_orm::{Database, DbConn, EntityTrait};
use serial_test::serial;
use uuid::Uuid;

fn new_email() -> String {
    format!("recipient-{}@example.com", Uuid::new_v4())
}

async fn setup_db() -> (DbConn, AppConfig) {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db = Database::connect(&config.database_test_url).await.unwrap();

    email::Entity::delete_many().exec(&db).await.unwrap();

    (db, config)
}

#[tokio::test]
#[serial]
async fn test_save_sent_email_basic_insert() {
    let (db, _config) = setup_db().await;

    let args = SaveEmailArgs {
        recipient: new_email(),
        email_type: EmailType::Passcode,

        subject: Some("Your login code".into()),
        content: Some("Your passcode is: 1234".into()),

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

    let saved = save_sent_email(&db, args.clone()).await.expect("failed to insert");

    assert_eq!(saved.recipient, args.recipient);
    assert_eq!(saved.email_type, args.email_type);
    assert_eq!(saved.status, "sent");
    assert_eq!(saved.provider, Provider::Mailhog);
    assert_eq!(saved.retry_count, 0);
    assert_eq!(saved.sent_by_fallback, false);
    assert_eq!(saved.subject, args.subject);
    assert_eq!(saved.content, args.content);
    assert!(saved.created_at <= Utc::now());
    assert!(saved.updated_at <= Utc::now());
}

#[tokio::test]
#[serial]
async fn test_save_sent_email_with_error_message() {
    let (db, _config) = setup_db().await;

    let args = SaveEmailArgs {
        recipient: new_email(),
        email_type: EmailType::Passcode,

        subject: None,
        content: None,

        template_type: Some("plain".into()),
        template_language: Some("en".into()),
        rendered_with: Some("basic".into()),

        provider: Provider::Mailhog,
        status: "failed".into(),
        error_message: Some("SMTP timeout".into()),
        message_id: None,

        retry_count: 2,
        sent_by_fallback: true,
        opened_at: None,
    };

    let saved = save_sent_email(&db, args.clone()).await.unwrap();

    assert_eq!(saved.status, "failed");
    assert_eq!(saved.error_message, args.error_message);
    assert_eq!(saved.template_type, args.template_type);
    assert_eq!(saved.rendered_with, args.rendered_with);
}
