use emailer::validators::has_digits_only;

#[test]
fn test_valid_passcode_all_digits() {
    let passcode = "123456";
    assert!(has_digits_only(passcode).is_ok());
}

#[test]
fn test_valid_empty_passcode() {
    let passcode = "";
    // Empty string has no non-digits, so .all() over empty returns true — Ok.
    assert!(has_digits_only(passcode).is_ok());
}

#[test]
fn test_passcode_with_letters_fails() {
    let passcode = "12a4";
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_special_chars_fails() {
    let passcode = "1234!";
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_spaces_fails() {
    let passcode = "12 34";
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_unicode_digits_fails() {
    let passcode = "١٢٣٤"; // Arabic-Indic digits (U+0661 etc.)
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_mixed_unicode_and_ascii_digits_fails() {
    let passcode = "123٤"; // Last one is Arabic-Indic '4'
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_symbols_only_fails() {
    let passcode = "!!@#";
    assert!(has_digits_only(passcode).is_err());
}

#[test]
fn test_passcode_with_dot_fails() {
    let passcode = "123.456";
    assert!(has_digits_only(passcode).is_err());
}
