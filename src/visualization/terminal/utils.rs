use crate::profiler::ColumnProfile;

pub fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|w| *w >= 60)
        .unwrap_or(100)
}

pub fn supports_unicode() -> bool {
    if std::env::var("NO_UNICODE").is_ok() {
        return false;
    }
    let term = std::env::var("TERM").unwrap_or_default();
    if term == "dumb" {
        return false;
    }
    let lang = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_default();
    lang.to_ascii_lowercase().contains("utf-8")
}

pub fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        value.to_string()
    } else {
        let mut s = value
            .chars()
            .take(max.saturating_sub(3))
            .collect::<String>();
        s.push_str("...");
        s
    }
}

pub fn numeric_column_names(profiles: &[ColumnProfile]) -> Vec<&str> {
    profiles
        .iter()
        .filter(|p| p.mean.is_some() || p.min.is_some() || p.max.is_some())
        .map(|p| p.name.as_str())
        .collect()
}
