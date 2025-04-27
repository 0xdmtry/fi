pub fn normalize_string(raw: &str) -> String {
    raw.trim().replace(char::is_whitespace, "").to_lowercase()
}
