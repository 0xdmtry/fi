use transactor::utils::normalize::normalize_str;

#[test]
fn test_email_cases_normalization() {
    let input = " User1@Example.Com ";
    let output = normalize_str(input);

    assert_eq!(output, "user1@example.com");
}

#[test]
fn test_email_spaces_normalization() {
    let input = "u  s e r2@ e x      a m p    l e .      c o m           ";
    let output = normalize_str(input);

    assert_eq!(output, "user2@example.com");
}