// userland/lib/filesystem.rs

use std::fs::{self, File, OpenOptions, ReadDir};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// Представление открытого файла
pub struct FsFile {
    file: File,
}

impl FsFile {
    /// Открыть файл для чтения
    pub fn open_read<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(FsFile { file })
    }

    /// Открыть файл для записи (создать или перезаписать)
    pub fn open_write<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        Ok(FsFile { file })
    }

    /// Прочитать содержимое файла в строку
    pub fn read_to_string(&mut self) -> io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Записать строку в файл
    pub fn write_string(&mut self, data: &str) -> io::Result<()> {
        self.file.write_all(data.as_bytes())
    }
}

/// Функция для перечисления файлов и папок в директории
pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let entries: ReadDir = fs::read_dir(path)?;
    let mut paths = Vec::new();

    for entry in entries {
        let entry = entry?;
        paths.push(entry.path());
    }

    Ok(paths)
}

/// Создать новую директорию (если не существует)
pub fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

/// Удалить файл
pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path)
}

/// Удалить директорию и её содержимое рекурсивно
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_dir_all(path)
}

/// Проверка существования файла или директории
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
