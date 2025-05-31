use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// Максимальное количество команд в истории
const MAX_HISTORY_SIZE: usize = 100;

/// Файл для сохранения истории команд
const HISTORY_FILE: &str = ".boyjack_shell_history";

/// История команд — хранит последние команды в кольцевом буфере
pub struct History {
    commands: VecDeque<String>,
    file_path: PathBuf,
}

impl History {
    /// Создает новую историю, загружая команды из файла, если он существует
    pub fn new() -> io::Result<Self> {
        let mut commands = VecDeque::with_capacity(MAX_HISTORY_SIZE);
        let file_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(HISTORY_FILE);

        if file_path.exists() {
            let file = File::open(&file_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines().flatten() {
                if commands.len() == MAX_HISTORY_SIZE {
                    commands.pop_front();
                }
                commands.push_back(line);
            }
        }

        Ok(History { commands, file_path })
    }

    /// Добавляет команду в историю (если она не пустая и не дублирует последнюю)
    pub fn add_command(&mut self, cmd: String) {
        if cmd.trim().is_empty() {
            return;
        }
        if self.commands.back().map_or(false, |last| last == &cmd) {
            return;
        }
        if self.commands.len() == MAX_HISTORY_SIZE {
            self.commands.pop_front();
        }
        self.commands.push_back(cmd);
    }

    /// Сохраняет историю команд в файл
    pub fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;

        for cmd in &self.commands {
            writeln!(file, "{}", cmd)?;
        }
        Ok(())
    }

    /// Получить последнюю команду (если есть)
    pub fn last_command(&self) -> Option<&String> {
        self.commands.back()
    }

    /// Получить команду по индексу (0 — самая старая)
    pub fn get_command(&self, index: usize) -> Option<&String> {
        self.commands.get(index)
    }

    /// Получить все команды в истории
    pub fn all_commands(&self) -> impl Iterator<Item = &String> {
        self.commands.iter()
    }
}
