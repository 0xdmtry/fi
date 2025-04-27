use walletor::payloads::signup_payload::SignupRequest;
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_valid_signup_request_passes() {
    let model = SignupRequest {
        user_id: Uuid::new_v4(),
        passcode: "123456".to_string(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_passcode_with_letters_fails() {
    let model = SignupRequest {
        user_id: Uuid::new_v4(),
        passcode: "12a4".into(), // non-digits
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_too_short_fails() {
    let model = SignupRequest {
        user_id: Uuid::new_v4(),
        passcode: "12".into(), // < 4 characters
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_too_long_fails() {
    let model = SignupRequest {
        user_id: Uuid::new_v4(),
        passcode: "1".repeat(20), // > 16 characters
    };

    assert!(model.validate().is_err());
}
