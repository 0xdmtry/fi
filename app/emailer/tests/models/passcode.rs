use emailer::models::passcode::SendFailedPasscodeRequest;
use emailer::models::passcode::SendPasscodeRequest;
use emailer::models::passcode::SendSuccessPasscodeRequest;

use validator::Validate;

fn valid_email() -> String {
    "user@example.com".into()
}

#[test]
fn test_valid_payload_passes() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "123456".into(),
    };

    assert!(model.validate().is_ok());
}

//
// Email validation tests (copied from Authorizer JoinRequest)
//
#[test]
fn test_invalid_email_fails() {
    let model = SendPasscodeRequest {
        email: "not-an-email".to_string(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_email_fails() {
    let model = SendPasscodeRequest {
        email: "".to_string(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_short_fails() {
    let model = SendPasscodeRequest {
        email: "a@b".to_string(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_long_fails() {
    let long_email = format!("{}@example.com", "a".repeat(245));
    let model = SendPasscodeRequest {
        email: long_email,
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_at_fails() {
    let model = SendPasscodeRequest {
        email: "user@@example.com".into(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_dot_in_domain_fails() {
    let model = SendPasscodeRequest {
        email: "user@example..com".into(),
        passcode: "1234".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_space_in_domain_fails() {
    let model = SendPasscodeRequest {
        email: "user@ ex ample.com".into(),
        passcode: "1234".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_leading_dot_in_local_part_fails() {
    let model = SendPasscodeRequest {
        email: ".user@example.com".into(),
        passcode: "1234".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_exclamation_in_domain_fails() {
    let model = SendPasscodeRequest {
        email: "user@exam!mple.com".to_string(),
        passcode: "1234".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_non_english_characters_fails() {
    let model = SendPasscodeRequest {
        email: "почта@example.com".into(),
        passcode: "1234".into(),
    };

    assert!(model.validate().is_err());
}

//
// Passcode validation tests
//
#[test]
fn test_passcode_too_short_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "123".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_too_long_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "12345678901234567".into(), // >16 chars
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_passcode_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_exact_bounds_pass() {
    let min_model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "1234".into(),
    };

    let max_model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "1".repeat(16),
    };

    assert!(min_model.validate().is_ok());
    assert!(max_model.validate().is_ok());
}

#[test]
fn test_passcode_with_letters_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "12a4".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_with_symbols_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "12$4".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_with_whitespace_fails() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "12 4".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_passcode_all_digits_passes() {
    let model = SendPasscodeRequest {
        email: valid_email(),
        passcode: "098765".into(),
    };

    assert!(model.validate().is_ok());
}

// SendSuccessPasscodeRequest

#[test]
fn test_valid_success_email_passes() {
    let model = SendSuccessPasscodeRequest {
        email: valid_email(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_invalid_success_email_fails() {
    let model = SendSuccessPasscodeRequest {
        email: "not-an-email".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_success_email_fails() {
    let model = SendSuccessPasscodeRequest { email: "".into() };

    assert!(model.validate().is_err());
}

// Failed Passcode

#[test]
fn test_valid_failed_email_passes() {
    let model = SendFailedPasscodeRequest {
        email: valid_email(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_invalid_failed_email_fails() {
    let model = SendFailedPasscodeRequest {
        email: "not-an-email".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_failed_email_fails() {
    let model = SendFailedPasscodeRequest { email: "".into() };

    assert!(model.validate().is_err());
}
