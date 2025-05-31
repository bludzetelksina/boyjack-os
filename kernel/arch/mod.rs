//! Модуль выбора архитектуры процессора (ARMv7, x86_64 и др.)

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "arm")]
pub mod armv7;

// Более точная фильтрация для ARMv7 (если нужно)
#[cfg(all(target_arch = "arm", feature = "armv7"))]
pub mod armv7;

// Общие типы и функции для работы с архитектурой
pub use self::arch::*;

// Вспомогательный модуль для упрощения импорта
#[cfg(target_arch = "x86_64")]
mod arch {
    pub use super::x86_64::*;
}

#[cfg(all(target_arch = "arm", feature = "armv7"))]
mod arch {
    pub use super::armv7::*;
}
