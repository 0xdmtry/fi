use validator::ValidationError;

pub fn is_email_safe_format(email: &str) -> Result<(), ValidationError> {
    if let Some((local, _)) = email.split_once('@') {
        if local.starts_with('.') || local.ends_with('.') {
            return Err(ValidationError::new("invalid_local_part"));
        }
    }

    Ok(())
}
