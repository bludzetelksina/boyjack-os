#![no_std]

use core::ptr;

pub const PAGE_SIZE: usize = 0x1000;        // 4 КБ
pub const SECTION_SIZE: usize = 0x100000;   // 1 МБ секция для L1 таблиц ARMv7

pub const L1_TABLE_ENTRIES: usize = 4096;  // Кол-во записей в таблице L1 (по 1 МБ каждая)

/// Тип таблицы L1 (1-й уровень), выравненный на 16 КБ
#[repr(align(16384))]
pub struct L1Table([u32; L1_TABLE_ENTRIES]);

/// Глобальная таблица первого уровня (статически выделенная)
static mut L1_TABLE: L1Table = L1Table([0; L1_TABLE_ENTRIES]);

/// Инициализация таблицы страниц с identity mapping для первых 16 МБ памяти
pub unsafe fn init_memory_mapping() {
    let l1 = &mut L1_TABLE.0;

    // Простая identity mapping секциями по 1 МБ
    for i in 0..16 {
        l1[i] = section_entry(i * SECTION_SIZE as u32);
    }

    // Установить таблицу страниц (TTBR0) в регистр
    set_ttbr0(&L1_TABLE as *const _ as u32);

    // Включить MMU (упрощённо, детали зависят от конкретного ARM процессора)
    enable_mmu();
}

/// Создание записи секции L1 таблицы (Section Descriptor)
fn section_entry(phys_addr: u32) -> u32 {
    // Флаги:
    // bit0-1 = 0b10 (section)
    // AP bits = 0b11 (Full access)
    // Domain = 0
    // Cacheable и Bufferable включены (для примера)
    phys_addr & 0xFFF00000 | (0b10) | (0b11 << 10) | (1 << 3) | (1 << 2)
}

/// Установка TTBR0 (Translation Table Base Register 0)
unsafe fn set_ttbr0(addr: u32) {
    asm!(
        "mcr p15, 0, {0}, c2, c0, 0",
        in(reg) addr,
        options(nostack)
    );
}

/// Включение MMU (упрощённо)
unsafe fn enable_mmu() {
    let mut sctlr: u32;
    asm!(
        "mrc p15, 0, {0}, c1, c0, 0",  // чтение системного контроля
        out(reg) sctlr,
        options(nostack)
    );

    sctlr |= 1 << 0; // Включаем MMU (bit 0)

    asm!(
        "mcr p15, 0, {0}, c1, c0, 0",  // запись обратно
        in(reg) sctlr,
        options(nostack)
    );
}
