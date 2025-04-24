use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn is_email_safe_format(email: &str) -> Result<(), ValidationError> {
    if let Some((local, _)) = email.split_once('@') {
        if local.starts_with('.') || local.ends_with('.') {
            return Err(ValidationError::new("invalid_local_part"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct JoinRequest {
    #[validate(email)]
    #[validate(length(min = 5, max = 254))]
    #[validate(custom(function = "is_email_safe_format"))]
    pub email: String,
}
