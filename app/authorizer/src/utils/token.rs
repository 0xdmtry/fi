use crate::config::AppConfig;
use crate::models::user;
use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use chrono::{Utc, Duration};


#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub fn generate_jwt(config: &AppConfig, user: &user::Model) -> Result<String> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(config.access_token_ttl_seconds);

    let claims = Claims {
        sub: user.id.to_string(),
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}