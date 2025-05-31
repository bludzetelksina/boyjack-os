#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub input_redirection: Option<String>,  // < filename
    pub output_redirection: Option<String>, // > filename
}

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Vec<Command>, // Список команд, связанных пайпом
}

impl Pipeline {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

/// Парсит строку командной строки в структуру Pipeline
pub fn parse_line(line: &str) -> Result<Pipeline, String> {
    let mut pipeline = Pipeline::new();

    // Разбиваем строку по пайпам (|)
    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

    for part in parts {
        // Парсим отдельную команду
        let command = parse_command(part)?;
        pipeline.commands.push(command);
    }

    Ok(pipeline)
}

/// Парсит одну команду с аргументами и перенаправлениями
fn parse_command(command_str: &str) -> Result<Command, String> {
    let mut tokens = shell_split(command_str)?;

    let mut input_redirection = None;
    let mut output_redirection = None;

    // Ищем перенаправления в токенах
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "<" => {
                if i + 1 >= tokens.len() {
                    return Err("Expected filename after '<'".into());
                }
                input_redirection = Some(tokens.remove(i + 1));
                tokens.remove(i); // удаляем '<'
                continue; // не увеличиваем i, т.к. элементы сдвинулись
            }
            ">" => {
                if i + 1 >= tokens.len() {
                    return Err("Expected filename after '>'".into());
                }
                output_redirection = Some(tokens.remove(i + 1));
                tokens.remove(i);
                continue;
            }
            _ => i += 1,
        }
    }

    if tokens.is_empty() {
        return Err("Empty command".into());
    }

    let program = tokens.remove(0);
    let args = tokens;

    Ok(Command {
        program,
        args,
        input_redirection,
        output_redirection,
    })
}

/// Простейшая функция для разбиения строки на токены с поддержкой кавычек
fn shell_split(s: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '"' => {
                chars.next();
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                chars.next();
            }
            _ => {
                current.push(c);
                chars.next();
            }
        }
    }

    if in_quotes {
        return Err("Unmatched quotes".into());
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let line = "ls -la";
        let pipeline = parse_line(line).unwrap();
        assert_eq!(pipeline.commands.len(), 1);
        let cmd = &pipeline.commands[0];
        assert_eq!(cmd.program, "ls");
        assert_eq!(cmd.args, vec!["-la"]);
        assert!(cmd.input_redirection.is_none());
        assert!(cmd.output_redirection.is_none());
    }

    #[test]
    fn test_parse_redirection() {
        let line = "cat < input.txt > output.txt";
        let pipeline = parse_line(line).unwrap();
        let cmd = &pipeline.commands[0];
        assert_eq!(cmd.program, "cat");
        assert_eq!(cmd.input_redirection.as_deref(), Some("input.txt"));
        assert_eq!(cmd.output_redirection.as_deref(), Some("output.txt"));
    }

    #[test]
    fn test_parse_pipeline() {
        let line = "cat file.txt | grep hello | wc -l";
        let pipeline = parse_line(line).unwrap();
        assert_eq!(pipeline.commands.len(), 3);
        assert_eq!(pipeline.commands[0].program, "cat");
        assert_eq!(pipeline.commands[1].program, "grep");
        assert_eq!(pipeline.commands[2].program, "wc");
        assert_eq!(pipeline.commands[2].args, vec!["-l"]);
    }

    #[test]
    fn test_unmatched_quotes() {
        let line = "echo \"hello";
        assert!(parse_line(line).is_err());
    }
}
