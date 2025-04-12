pub fn normalize_email(raw: &str) -> String {
    raw.trim().replace(char::is_whitespace, "").to_lowercase()
}