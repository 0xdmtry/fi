pub fn normalize_str(raw: &str) -> String {
    raw.trim().replace(char::is_whitespace, "").to_lowercase()
}