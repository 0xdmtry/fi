use validator::ValidationError;

pub fn has_digits_only(passcode: &str) -> Result<(), ValidationError> {
    if passcode.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(ValidationError::new("passcode_must_be_digits_only"))
    }
}
