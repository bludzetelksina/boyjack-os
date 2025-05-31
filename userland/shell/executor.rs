use std::process::{Command as SysCommand, Stdio};
use std::io::{self, Write};
use std::fs::File;
use crate::parser::{Pipeline, Command};

/// Выполняет весь пайплайн команд
pub fn execute_pipeline(pipeline: &Pipeline) -> io::Result<()> {
    if pipeline.commands.is_empty() {
        return Ok(());
    }

    // Если одна команда — просто выполняем её
    if pipeline.commands.len() == 1 {
        return execute_command(&pipeline.commands[0]);
    }

    // Для пайпа создаём цепочку процессов с передачей вывода вводу следующей
    let mut previous_process = None;

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        let stdin = if i == 0 {
            // Первая команда может иметь входное перенаправление
            if let Some(ref input_file) = cmd.input_redirection {
                File::open(input_file)?
            } else {
                // стандартный ввод
                Stdio::inherit().into()
            }
        } else {
            // Ввод из предыдущего процесса
            previous_process.unwrap().stdout.unwrap()
        };

        let stdout = if i == pipeline.commands.len() - 1 {
            // Последняя команда может иметь выходное перенаправление
            if let Some(ref output_file) = cmd.output_redirection {
                File::create(output_file)?.into()
            } else {
                Stdio::inherit()
            }
        } else {
            // Создаём конвейер (pipe)
            Stdio::piped()
        };

        // Проверка на встроенную команду
        if is_builtin(cmd) {
            // Встроенные команды не поддерживают пайпы в этом примере
            if pipeline.commands.len() > 1 {
                eprintln!("Встроенные команды не поддерживают пайпы");
                return Ok(());
            }
            return execute_builtin(cmd);
        }

        // Запускаем внешний процесс
        let mut process = SysCommand::new(&cmd.program)
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?;

        previous_process = Some(process);
    }

    // Ожидаем завершения последнего процесса
    if let Some(mut process) = previous_process {
        process.wait()?;
    }

    Ok(())
}

/// Проверка, является ли команда встроенной
fn is_builtin(cmd: &Command) -> bool {
    matches!(cmd.program.as_str(), "cd" | "exit" | "help")
}

/// Выполнение встроенных команд
fn execute_builtin(cmd: &Command) -> io::Result<()> {
    match cmd.program.as_str() {
        "cd" => {
            let path = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("/");
            std::env::set_current_dir(path)?;
        }
        "exit" => {
            std::process::exit(0);
        }
        "help" => {
            println!("Встроенные команды: cd, exit, help");
        }
        _ => {
            eprintln!("Неизвестная встроенная команда: {}", cmd.program);
        }
    }
    Ok(())
}

/// Выполняет отдельную команду (без пайпов)
fn execute_command(cmd: &Command) -> io::Result<()> {
    if is_builtin(cmd) {
        return execute_builtin(cmd);
    }

    // Настраиваем перенаправления
    let stdin = if let Some(ref input_file) = cmd.input_redirection {
        File::open(input_file)?
    } else {
        Stdio::inherit().into()
    };

    let stdout = if let Some(ref output_file) = cmd.output_redirection {
        File::create(output_file)?.into()
    } else {
        Stdio::inherit()
    };

    let mut child = SysCommand::new(&cmd.program)
        .args(&cmd.args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()?;

    child.wait()?;
    Ok(())
}
