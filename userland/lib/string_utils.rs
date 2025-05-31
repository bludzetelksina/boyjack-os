// userland/lib/string_utils.rs

/// Разделяет строку по разделителю и возвращает вектор подстрок
pub fn split_string(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(|part| part.to_string()).collect()
}

/// Проверяет, содержит ли строка подстроку
pub fn contains_substring(s: &str, substring: &str) -> bool {
    s.contains(substring)
}

/// Преобразует строку в нижний регистр
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// Преобразует строку в верхний регистр
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// Удаляет пробелы в начале и конце строки
pub fn trim_whitespace(s: &str) -> String {
    s.trim().to_string()
}

/// Проверяет, начинается ли строка с заданного префикса
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Проверяет, заканчивается ли строка на заданный суффикс
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Заменяет все вхождения подстроки `from` на `to`
pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// Разворачивает строку задом наперёд
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let result = split_string("one,two,three", ',');
        assert_eq!(result, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_contains_substring() {
        assert!(contains_substring("hello world", "world"));
        assert!(!contains_substring("hello world", "mars"));
    }

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase("HeLLo"), "hello");
    }

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("HeLLo"), "HELLO");
    }

    #[test]
    fn test_trim_whitespace() {
        assert_eq!(trim_whitespace("  hello "), "hello");
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with("hello world", "hello"));
        assert!(!starts_with("hello world", "world"));
    }

    #[test]
    fn test_ends_with() {
        assert!(ends_with("hello world", "world"));
        assert!(!ends_with("hello world", "hello"));
    }

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("foo bar foo", "foo", "baz"), "baz bar baz");
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("abc"), "cba");
    }
}
