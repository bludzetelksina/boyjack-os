use std::{thread, time::Duration};

// Имитация получения данных системы
struct SystemStats {
    cpu_usage_percent: f32,
    memory_used_mb: u32,
    memory_total_mb: u32,
    uptime_seconds: u64,
}

fn get_system_stats() -> SystemStats {
    // Здесь в реальной OS будут вызовы к ядру и драйверам,
    // сейчас заглушка с рандомными/статическими значениями
    SystemStats {
        cpu_usage_percent: 23.5,
        memory_used_mb: 512,
        memory_total_mb: 1024,
        uptime_seconds: 3600 * 5 + 42, // 5 часов 42 секунды
    }
}

fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}ч {:02}м {:02}с", hours, minutes, seconds)
}

fn main() {
    println!("Системный монитор BoyJack OS");
    println!("Нажмите Ctrl+C для выхода\n");

    loop {
        let stats = get_system_stats();

        println!("--- Статистика системы ---");
        println!("Загрузка CPU: {:.1}%", stats.cpu_usage_percent);
        println!("Память: {} MB из {} MB используется", stats.memory_used_mb, stats.memory_total_mb);
        println!("Время работы системы: {}", format_uptime(stats.uptime_seconds));

        println!("\nОбновление через 3 секунды...\n");

        thread::sleep(Duration::from_secs(3));

        // Очистка экрана (ANSI escape code)
        print!("\x1B[2J\x1B[1;1H");
    }
}
