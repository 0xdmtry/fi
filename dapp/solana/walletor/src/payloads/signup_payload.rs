use crate::validators::passcode_validator::has_digits_only;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupRequest {
    pub user_id: Uuid,

    #[validate(length(min = 4, max = 16))]
    #[validate(custom(function = "has_digits_only"))]
    pub passcode: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupResponse {
    pub wallet_id: Uuid,
    pub wallet_address: String,
    pub wallet_public_key: String,
}
