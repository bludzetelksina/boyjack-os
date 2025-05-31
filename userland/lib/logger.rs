// userland/lib/logger.rs

use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Уровни логирования
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

/// Глобальный уровень логирования (по умолчанию Info)
static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::Info as usize);

/// Устанавливает глобальный уровень логирования
pub fn set_level(level: LogLevel) {
    LOG_LEVEL.store(level as usize, Ordering::Relaxed);
}

/// Получает текущий глобальный уровень логирования
pub fn level() -> LogLevel {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        3 => LogLevel::Info,
        4 => LogLevel::Debug,
        _ => LogLevel::Info,
    }
}

/// Логгирует сообщение с указанным уровнем
pub fn log(level: LogLevel, msg: &str) {
    if level as usize > LOG_LEVEL.load(Ordering::Relaxed) {
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let seconds = now.as_secs();
    let millis = now.subsec_millis();

    let level_str = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => " WARN",
        LogLevel::Info => " INFO",
        LogLevel::Debug => "DEBUG",
    };

    // Выводим в stderr с форматом: [секунды.миллисекунды][LEVEL] сообщение
    let _ = writeln!(
        io::stderr(),
        "[{:>10}.{:03}][{}] {}",
        seconds, millis, level_str, msg
    );
}

/// Макросы для удобного логгирования на разных уровнях

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::logger::log($crate::logger::LogLevel::Error, &format!($($arg)*));
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::logger::log($crate::logger::LogLevel::Warn, &format!($($arg)*));
    })
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::logger::log($crate::logger::LogLevel::Info, &format!($($arg)*));
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::logger::log($crate::logger::LogLevel::Debug, &format!($($arg)*));
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        set_level(LogLevel::Debug);

        log(LogLevel::Error, "Error level test");
        log(LogLevel::Warn, "Warn level test");
        log(LogLevel::Info, "Info level test");
        log(LogLevel::Debug, "Debug level test");
    }

    #[test]
    fn test_log_macros() {
        set_level(LogLevel::Info);

        error!("This is an error: {}", 42);
        warn!("This is a warning");
        info!("Information here");
        debug!("This debug message should not appear at Info level");
    }
}
