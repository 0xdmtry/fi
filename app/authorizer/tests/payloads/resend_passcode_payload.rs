use authorizer::payloads::resend_passcode_payload::ResendRequest;
use validator::Validate;

#[test]
fn test_valid_email_passes() {
    let model = ResendRequest {
        email: "user@example.com".to_string(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_invalid_email_fails() {
    let model = ResendRequest {
        email: "not-an-email".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_email_fails() {
    let model = ResendRequest {
        email: "".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_short_fails() {
    let model = ResendRequest {
        email: "a@b".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_long_fails() {
    let long_email = format!("{}@example.com", "a".repeat(245));
    let model = ResendRequest { email: long_email };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_at_fails() {
    let model = ResendRequest {
        email: "user@@example.com".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_dot_in_domain_fails() {
    let model = ResendRequest {
        email: "user@example..com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_space_in_domain_fails() {
    let model = ResendRequest {
        email: "user@ ex ample.com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_leading_dot_in_local_part_fails() {
    let model = ResendRequest {
        email: ".user@example.com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_exclamation_in_domain_fails() {
    let model = ResendRequest {
        email: "user@exam!mple.com".to_string(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_non_english_characters_fails() {
    let model = ResendRequest {
        email: "почта@example.com".into(),
    };

    assert!(model.validate().is_err());
}
