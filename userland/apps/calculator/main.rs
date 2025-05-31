use std::io::{self, Write};

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Очень простой парсер и вычислитель выражений с двумя операндами и одним оператором
    // Поддерживаются: +, -, *, /
    // Например: "3 + 4", "10 / 2"

    let tokens: Vec<&str> = expr.trim().split_whitespace().collect();
    if tokens.len() != 3 {
        return Err("Ошибка: выражение должно быть в формате <число> <оператор> <число>".to_string());
    }

    let left = tokens[0].parse::<f64>().map_err(|_| "Ошибка: левый операнд не число".to_string())?;
    let op = tokens[1];
    let right = tokens[2].parse::<f64>().map_err(|_| "Ошибка: правый операнд не число".to_string())?;

    let result = match op {
        "+" => left + right,
        "-" => left - right,
        "*" | "x" => left * right,
        "/" => {
            if right == 0.0 {
                return Err("Ошибка: деление на ноль".to_string());
            }
            left / right
        }
        _ => return Err("Ошибка: неизвестный оператор. Поддерживаются +, -, *, /".to_string()),
    };

    Ok(result)
}

fn main() {
    println!("Калькулятор BoyJack OS");
    println!("Введите выражение в формате: <число> <оператор> <число>");
    println!("Поддерживаемые операторы: + - * /");
    println!("Для выхода введите 'exit'");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Выход...");
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Результат: {}", result),
            Err(e) => println!("{}", e),
        }
    }
}
