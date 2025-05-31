// userland/lib/io.rs

use std::io::{self, Read, Write, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;

/// Чтение всего содержимого из файла в строку
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

/// Запись строки в файл (создаёт или перезаписывает)
pub fn write_string_to_file<P: AsRef<Path>>(path: P, data: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(data.as_bytes())?;
    buf_writer.flush()
}

/// Чтение из стандартного ввода до нажатия Enter
pub fn read_line_from_stdin() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end().to_string())
}

/// Вывод строки в стандартный вывод с переводом строки
pub fn println_stdout(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(s.as_bytes())?;
    handle.write_all(b"\n")?;
    handle.flush()
}

/// Вывод строки в стандартный вывод без перевода строки
pub fn print_stdout(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(s.as_bytes())?;
    handle.flush()
}

/// Буферизированный ввод из любого `Read`
pub fn buffered_read<R: Read>(reader: &mut R, buffer: &mut [u8]) -> io::Result<usize> {
    let mut buf_reader = BufReader::new(reader);
    buf_reader.read(buffer)
}

/// Буферизированный вывод в любой `Write`
pub fn buffered_write<W: Write>(writer: &mut W, data: &[u8]) -> io::Result<()> {
    let mut buf_writer = BufWriter::new(writer);
    buf_writer.write_all(data)?;
    buf_writer.flush()
}
