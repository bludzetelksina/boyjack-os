#![no_std]
#![no_main]

use core::ptr::Unique;

const PAGE_SIZE: usize = 4096;
const ENTRY_COUNT: usize = 512;

bitflags::bitflags! {
    pub struct PageTableFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const CACHE_DISABLE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const HUGE_PAGE = 1 << 7;
        const GLOBAL = 1 << 8;
        const NO_EXECUTE = 1 << 63;
    }
}

#[repr(align(4096))]
pub struct PageTable {
    entries: [u64; ENTRY_COUNT],
}

impl PageTable {
    pub fn zero(&mut self) {
        for i in 0..ENTRY_COUNT {
            self.entries[i] = 0;
        }
    }

    pub fn set_entry(&mut self, index: usize, addr: u64, flags: PageTableFlags) {
        self.entries[index] = (addr & 0x000fffff_fffff000) | flags.bits();
    }
}

// Корневой уровень таблицы страниц PML4
static mut PML4: PageTable = PageTable {
    entries: [0; ENTRY_COUNT],
};

/// Инициализация физической памяти (простейшая, для примера)
pub unsafe fn init_physical_memory() {
    PML4.zero();

    // Пример маппинга первого 1 MiB физической памяти в виртуальное пространство (identity mapping)
    // В реальной ОС нужно будет создавать уровни таблиц (PDPT, PD, PT)
    // Здесь упрощенно, для демонстрации

    // TODO: Реализовать создание всех уровней таблиц и маппинг страниц
}

/// Простая функция для переключения активной таблицы страниц (PML4)
pub unsafe fn load_pml4_table(pml4_addr: u64) {
    asm!(
        "mov cr3, {0}",
        in(reg) pml4_addr,
        options(nostack, preserves_flags),
    );
}
