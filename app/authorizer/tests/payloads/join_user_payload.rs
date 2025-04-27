use authorizer::payloads::join_user_payload::JoinRequest;
use validator::Validate;

#[test]
fn test_valid_email_passes() {
    let model = JoinRequest {
        email: "user@example.com".to_string(),
    };

    assert!(model.validate().is_ok());
}

#[test]
fn test_invalid_email_fails() {
    let model = JoinRequest {
        email: "not-an-email".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_empty_email_fails() {
    let model = JoinRequest {
        email: "".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_short_fails() {
    let model = JoinRequest {
        email: "a@b".to_string(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_too_long_fails() {
    let long_email = format!("{}@example.com", "a".repeat(245));
    let model = JoinRequest { email: long_email };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_at_fails() {
    let model = JoinRequest {
        email: "user@@example.com".into(),
    };

    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_double_dot_in_domain_fails() {
    let model = JoinRequest {
        email: "user@example..com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_space_in_domain_fails() {
    let model = JoinRequest {
        email: "user@ ex ample.com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_leading_dot_in_local_part_fails() {
    let model = JoinRequest {
        email: ".user@example.com".into(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_exclamation_in_domain_fails() {
    let model = JoinRequest {
        email: "user@exam!mple.com".to_string(),
    };
    assert!(model.validate().is_err());
}

#[test]
fn test_email_with_non_english_characters_fails() {
    let model = JoinRequest {
        email: "почта@example.com".into(),
    };

    assert!(model.validate().is_err());
}
