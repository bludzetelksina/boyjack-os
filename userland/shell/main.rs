use std::io::{self, Write};

fn main() {
    println!("Welcome to BoyJack OS!");
    
    loop {
        // Вывод приглашения
        print!("boyjack> ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        // Чтение команды с stdin
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    // EOF — выходим из shell
                    println!("\nExiting shell.");
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        }
        
        // Удаляем пробельные символы по краям
        let command = input.trim();
        
        if command.is_empty() {
            continue;
        }
        
        // Обработка встроенных команд
        match command {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  help  - show this help");
                println!("  exit  - exit the shell");
                println!("  echo  - print arguments");
                println!("  clear - clear the screen");
            }
            cmd if cmd.starts_with("echo ") => {
                // echo с выводом аргументов
                let echo_text = &cmd[5..];
                println!("{}", echo_text);
            }
            "clear" => {
                // Очистка экрана ANSI-escape последовательностью
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
