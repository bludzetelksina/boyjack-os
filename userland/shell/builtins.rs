use std::env;
use std::io::{self, Write};
use std::process;

/// Тип результата выполнения встроенной команды
pub type BuiltinResult = io::Result<()>;

/// Выполняет встроенную команду с аргументами
pub fn run_builtin(command: &str, args: &[String]) -> BuiltinResult {
    match command {
        "cd" => builtin_cd(args),
        "exit" => builtin_exit(args),
        "help" => builtin_help(args),
        "pwd" => builtin_pwd(args),
        "echo" => builtin_echo(args),
        "setenv" => builtin_setenv(args),
        "unsetenv" => builtin_unsetenv(args),
        _ => {
            writeln!(io::stderr(), "Неизвестная встроенная команда: {}", command)?;
            Ok(())
        }
    }
}

/// Команда `cd` — смена текущей директории
fn builtin_cd(args: &[String]) -> BuiltinResult {
    let target = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };

    if let Err(e) = env::set_current_dir(&target) {
        writeln!(io::stderr(), "cd: не удалось перейти в '{}': {}", target, e)?;
    }
    Ok(())
}

/// Команда `exit` — выход из оболочки
fn builtin_exit(args: &[String]) -> BuiltinResult {
    let code = if !args.is_empty() {
        args[0].parse::<i32>().unwrap_or(0)
    } else {
        0
    };
    process::exit(code);
}

/// Команда `help` — выводит список встроенных команд
fn builtin_help(_args: &[String]) -> BuiltinResult {
    let help_text = "\
Встроенные команды:
  cd [dir]       - сменить текущую директорию
  exit [code]    - выйти из оболочки с кодом возврата
  help           - показать это сообщение
  pwd            - вывести текущую директорию
  echo [args...] - вывести аргументы
  setenv VAR VAL - задать переменную окружения
  unsetenv VAR   - удалить переменную окружения
";
    println!("{}", help_text);
    Ok(())
}

/// Команда `pwd` — печать текущей директории
fn builtin_pwd(_args: &[String]) -> BuiltinResult {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => writeln!(io::stderr(), "pwd: ошибка: {}", e)?,
    }
    Ok(())
}

/// Команда `echo` — вывод аргументов
fn builtin_echo(args: &[String]) -> BuiltinResult {
    println!("{}", args.join(" "));
    Ok(())
}

/// Команда `setenv` — установить переменную окружения
fn builtin_setenv(args: &[String]) -> BuiltinResult {
    if args.len() < 2 {
        writeln!(io::stderr(), "setenv: недостаточно аргументов")?;
    } else {
        env::set_var(&args[0], &args[1]);
    }
    Ok(())
}

/// Команда `unsetenv` — удалить переменную окружения
fn builtin_unsetenv(args: &[String]) -> BuiltinResult {
    if args.is_empty() {
        writeln!(io::stderr(), "unsetenv: не указано имя переменной")?;
    } else {
        env::remove_var(&args[0]);
    }
    Ok(())
}
