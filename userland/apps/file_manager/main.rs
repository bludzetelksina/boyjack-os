mod commands;
mod utils;

use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    println!("BoyJack OS File Manager");
    let mut current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

    loop {
        print!("fm:{}> ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match cmd {
            "ls" => commands::list_dir(&current_dir),
            "cd" => {
                if args.is_empty() {
                    println!("Использование: cd <путь>");
                } else {
                    if let Some(new_dir) = commands::change_dir(&current_dir, args[0]) {
                        current_dir = new_dir;
                    } else {
                        println!("Путь не найден или недоступен");
                    }
                }
            }
            "rm" => {
                if args.is_empty() {
                    println!("Использование: rm <файл/каталог>");
                } else if let Err(e) = commands::remove(&current_dir, args[0]) {
                    println!("Ошибка удаления: {}", e);
                }
            }
            "cp" => {
                if args.len() < 2 {
                    println!("Использование: cp <исходник> <назначение>");
                } else if let Err(e) = commands::copy(&current_dir, args[0], args[1]) {
                    println!("Ошибка копирования: {}", e);
                }
            }
            "exit" => break,
            "help" => {
                println!("Команды:");
                println!("  ls                - список файлов");
                println!("  cd <путь>         - сменить каталог");
                println!("  rm <файл/каталог> - удалить");
                println!("  cp <src> <dest>   - копировать");
                println!("  exit              - выход");
            }
            _ => println!("Неизвестная команда. Введите help для списка команд."),
        }
    }

    println!("Выход из File Manager");
}
