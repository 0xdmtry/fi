use walletor::validators::is_email_safe_format;

#[test]
fn test_valid_email_format() {
    let email = "user@example.com";
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_valid_email_with_subdomain() {
    let email = "user@mail.example.com";
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_invalid_email_starts_with_dot() {
    let email = ".user@example.com";
    assert!(is_email_safe_format(email).is_err());
}

#[test]
fn test_invalid_email_ends_with_dot() {
    let email = "user.@example.com";
    assert!(is_email_safe_format(email).is_err());
}

#[test]
fn test_valid_email_with_dot_inside_local_part() {
    let email = "first.last@example.com";
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_email_without_at_symbol() {
    let email = "userexample.com";
    // No '@', so `split_once` will not trigger local part checks — expected OK.
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_empty_email() {
    let email = "";
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_email_only_at_symbol() {
    let email = "@";
    assert!(is_email_safe_format(email).is_ok());
}

#[test]
fn test_email_local_part_only_dot() {
    let email = ".@example.com";
    assert!(is_email_safe_format(email).is_err());
}

#[test]
fn test_email_local_part_ends_with_dot() {
    let email = "user.@example.com";
    assert!(is_email_safe_format(email).is_err());
}

#[test]
fn test_email_local_part_starts_and_ends_with_dot() {
    let email = ".user.@example.com";
    assert!(is_email_safe_format(email).is_err());
}

#[test]
fn test_email_with_multiple_at_symbols() {
    let email = "user@@example.com";
    // `split_once` splits at the first '@', so local="user", domain="@example.com"
    // local does not start/end with a dot, so OK.
    assert!(is_email_safe_format(email).is_ok());
}
