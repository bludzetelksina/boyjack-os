/// Модуль автодополнения для оболочки BoyJack OS

/// Структура для автодополнения
pub struct Completer {
    /// Список известных команд для автодополнения
    commands: Vec<String>,
}

impl Completer {
    /// Создать новый Completer с набором команд
    pub fn new(commands: Vec<String>) -> Self {
        Completer { commands }
    }

    /// Получить список возможных автодополнений по префиксу `input`
    pub fn complete(&self, input: &str) -> Vec<String> {
        let input_lower = input.to_lowercase();
        self.commands
            .iter()
            .filter(|cmd| cmd.to_lowercase().starts_with(&input_lower))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_empty_input() {
        let completer = Completer::new(vec![
            "help".into(),
            "exit".into(),
            "list".into(),
            "load".into(),
        ]);
        let completions = completer.complete("");
        assert_eq!(completions.len(), 4);
    }

    #[test]
    fn test_complete_partial() {
        let completer = Completer::new(vec![
            "help".into(),
            "exit".into(),
            "list".into(),
            "load".into(),
        ]);
        let completions = completer.complete("l");
        assert_eq!(completions, vec!["list".to_string(), "load".to_string()]);
    }

    #[test]
    fn test_complete_no_match() {
        let completer = Completer::new(vec![
            "help".into(),
            "exit".into(),
            "list".into(),
            "load".into(),
        ]);
        let completions = completer.complete("xyz");
        assert!(completions.is_empty());
    }
}
