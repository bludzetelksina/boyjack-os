// userland/apps/mod.rs

pub mod file_manager;
pub mod text_editor;
pub mod network_tool;
pub mod calculator;
pub mod system_monitor;

// Тип результата запуска приложения
pub type AppResult = Result<(), String>;

// Перечисление доступных приложений
pub enum App {
    FileManager,
    TextEditor,
    NetworkTool,
    Calculator,
    SystemMonitor,
}

impl App {
    // Запуск приложения по enum-значению
    pub fn run(&self, args: &[&str]) -> AppResult {
        match self {
            App::FileManager => file_manager::run(args),
            App::TextEditor => text_editor::run(args),
            App::NetworkTool => network_tool::run(args),
            App::Calculator => calculator::run(args),
            App::SystemMonitor => system_monitor::run(args),
        }
    }
}

// Функция для запуска приложения по имени из строки
pub fn run_app_by_name(name: &str, args: &[&str]) -> AppResult {
    let app = match name.to_lowercase().as_str() {
        "file_manager" | "fm" => App::FileManager,
        "text_editor" | "editor" | "te" => App::TextEditor,
        "network_tool" | "net" => App::NetworkTool,
        "calculator" | "calc" => App::Calculator,
        "system_monitor" | "sysmon" => App::SystemMonitor,
        _ => return Err(format!("Приложение '{}' не найдено", name)),
    };

    app.run(args)
}
