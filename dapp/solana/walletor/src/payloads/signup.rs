use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub user_id: Uuid,
    pub passcode: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub wallet_id: Uuid,
    pub wallet_address: String,
    pub wallet_public_key: String,
}
