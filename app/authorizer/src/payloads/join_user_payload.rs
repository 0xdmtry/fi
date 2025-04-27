use crate::validators::email_validator::is_email_safe_format;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct JoinRequest {
    #[validate(email)]
    #[validate(length(min = 5, max = 254))]
    #[validate(custom(function = "is_email_safe_format"))]
    pub email: String,
}
