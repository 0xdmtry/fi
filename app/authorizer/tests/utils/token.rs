use authorizer::config::AppConfig;
use authorizer::models::user::Model as UserModel;
use authorizer::utils::token::generate_jwt;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[test]
fn test_generate_jwt_should_include_expected_claims() {
    let config = AppConfig::from_env_with_custom_file(".test.env");

    let user = UserModel {
        id: Uuid::new_v4(),
        email: "user@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let token = generate_jwt(&config, &user).expect("JWT generation failed");

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .expect("Token should decode");

    assert_eq!(decoded.claims.sub, user.id.to_string());

    let now = Utc::now().timestamp() as usize;
    let leeway = 10; // seconds

    assert!(
        decoded.claims.iat >= now - leeway && decoded.claims.iat <= now + leeway,
        "Issued-at (iat) should be current"
    );

    assert!(
        decoded.claims.exp > now,
        "Expiration (exp) should be in the future"
    );
}
