use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Editor {
    lines: Vec<String>,
    filename: Option<PathBuf>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            filename: None,
        }
    }

    pub fn open(filename: &str) -> io::Result<Self> {
        let content = fs::read_to_string(filename)?;
        let lines = content.lines().map(String::from).collect();
        Ok(Self {
            lines,
            filename: Some(PathBuf::from(filename)),
        })
    }

    pub fn display(&self) {
        println!("--- Текстовый файл ---");
        for (i, line) in self.lines.iter().enumerate() {
            println!("{:>4}: {}", i + 1, line);
        }
        println!("---------------------");
    }

    pub fn edit_line(&mut self, line_num: usize, new_text: &str) -> bool {
        if line_num == 0 || line_num > self.lines.len() {
            false
        } else {
            self.lines[line_num - 1] = new_text.to_string();
            true
        }
    }

    pub fn insert_line(&mut self, text: String) {
        self.lines.push(text);
    }

    pub fn delete_line(&mut self, line_num: usize) -> bool {
        if line_num == 0 || line_num > self.lines.len() {
            false
        } else {
            self.lines.remove(line_num - 1);
            true
        }
    }

    pub fn save(&self) -> io::Result<()> {
        if let Some(ref filename) = self.filename {
            self.save_as(filename.to_str().unwrap())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Файл не задан"))
        }
    }

    pub fn save_as(&self, filename: &str) -> io::Result<()> {
        let mut file = fs::File::create(filename)?;
        for line in &self.lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}
