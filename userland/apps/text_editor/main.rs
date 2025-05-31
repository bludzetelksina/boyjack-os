mod editor;
mod utils;

use std::env;
use std::io::{self, Write};

fn main() {
    println!("BoyJack OS Text Editor");

    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    let mut editor = if let Some(ref file) = filename {
        match editor::Editor::open(file) {
            Ok(ed) => ed,
            Err(e) => {
                eprintln!("Ошибка открытия файла {}: {}", file, e);
                editor::Editor::new()
            }
        }
    } else {
        editor::Editor::new()
    };

    loop {
        editor.display();
        print!("Команда (help для справки): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода");
            continue;
        }
        let input = input.trim();

        match input {
            "help" => {
                println!("Команды:");
                println!("  edit <строка>      - редактировать строку");
                println!("  insert <строка>    - вставить строку");
                println!("  delete <номер>     - удалить строку");
                println!("  save               - сохранить");
                println!("  saveas <файл>      - сохранить в другой файл");
                println!("  exit               - выход");
            }
            cmd if cmd.starts_with("edit ") => {
                if let Some(text) = cmd.strip_prefix("edit ") {
                    println!("Введите номер строки для редактирования:");
                    let mut line_num_str = String::new();
                    io::stdin().read_line(&mut line_num_str).unwrap();
                    if let Ok(line_num) = line_num_str.trim().parse::<usize>() {
                        if editor.edit_line(line_num, text) {
                            println!("Строка {} изменена", line_num);
                        } else {
                            println!("Номер строки вне диапазона");
                        }
                    } else {
                        println!("Некорректный номер строки");
                    }
                }
            }
            cmd if cmd.starts_with("insert ") => {
                if let Some(text) = cmd.strip_prefix("insert ") {
                    editor.insert_line(text.to_string());
                    println!("Строка вставлена");
                }
            }
            cmd if cmd.starts_with("delete ") => {
                if let Some(num_str) = cmd.strip_prefix("delete ") {
                    if let Ok(line_num) = num_str.trim().parse::<usize>() {
                        if editor.delete_line(line_num) {
                            println!("Строка {} удалена", line_num);
                        } else {
                            println!("Номер строки вне диапазона");
                        }
                    } else {
                        println!("Некорректный номер строки");
                    }
                }
            }
            "save" => {
                if let Err(e) = editor.save() {
                    println!("Ошибка сохранения: {}", e);
                } else {
                    println!("Файл сохранён");
                }
            }
            cmd if cmd.starts_with("saveas ") => {
                if let Some(fname) = cmd.strip_prefix("saveas ") {
                    if let Err(e) = editor.save_as(fname) {
                        println!("Ошибка сохранения: {}", e);
                    } else {
                        println!("Файл сохранён как {}", fname);
                    }
                }
            }
            "exit" => break,
            _ => println!("Неизвестная команда. Введите help для списка."),
        }
    }

    println!("Выход из редактора");
}
