use crate::validators::email_validator::is_email_safe_format;
use crate::validators::passcode_validator::has_digits_only;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct VerifyPasscodeRequest {
    #[validate(email)]
    #[validate(length(min = 5, max = 254))]
    #[validate(custom(function = "is_email_safe_format"))]
    pub email: String,

    #[validate(length(min = 4, max = 16))]
    #[validate(custom(function = "has_digits_only"))]
    pub passcode: String,
}
