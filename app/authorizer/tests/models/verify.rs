use authorizer::models::verify::VerifyPasscodeRequest;
use validator::Validate;

#[test]
fn test_valid_verify_request_passes() {
    let model = VerifyPasscodeRequest {
        email: "user@example.com".to_string(),
        passcode: "123456".to_string(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_invalid_email_fails() {
    let model = VerifyPasscodeRequest {
        email: "invalid_email".into(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_leading_dot_fails() {
    let model = VerifyPasscodeRequest {
        email: ".user@example.com".into(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_short_fails() {
    let model = VerifyPasscodeRequest {
        email: "a@b".into(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_long_fails() {
    let long_email = format!("{}@example.com", "a".repeat(245));
    let model = VerifyPasscodeRequest {
        email: long_email,
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_with_letters_fails() {
    let model = VerifyPasscodeRequest {
        email: "user@example.com".into(),
        passcode: "12a4".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_too_short_fails() {
    let model = VerifyPasscodeRequest {
        email: "user@example.com".into(),
        passcode: "12".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_too_long_fails() {
    let model = VerifyPasscodeRequest {
        email: "user@example.com".into(),
        passcode: "1".repeat(20),
    };

    assert!(model.validate().is_err());
}
