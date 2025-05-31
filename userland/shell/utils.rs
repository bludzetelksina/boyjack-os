/// Вспомогательные функции для оболочки BoyJack OS

/// Обрезать пробелы с начала и конца строки
pub fn trim_whitespace(s: &str) -> &str {
    s.trim()
}

/// Разбить строку на слова, учитывая кавычки (например, "arg with spaces")
/// Возвращает вектор аргументов
pub fn split_command_line(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                chars.next(); // consume "
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
                chars.next();
            }
            _ => {
                current.push(ch);
                chars.next();
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }

    args
}

/// Проверка, является ли строка пустой или содержит только пробельные символы
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// Преобразовать строку в нижний регистр (UTF-8 безопасно)
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_whitespace() {
        assert_eq!(trim_whitespace("  hello "), "hello");
        assert_eq!(trim_whitespace("\t\n test \n"), "test");
    }

    #[test]
    fn test_split_command_line_basic() {
        let line = "echo hello world";
        let args = split_command_line(line);
        assert_eq!(args, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_split_command_line_with_quotes() {
        let line = "echo \"hello world\" test";
        let args = split_command_line(line);
        assert_eq!(args, vec!["echo", "hello world", "test"]);
    }

    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   \n\t"));
        assert!(!is_blank(" a "));
    }

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase("HeLLo"), "hello");
        assert_eq!(to_lowercase("БойДжек"), "бойджек");
    }
}
