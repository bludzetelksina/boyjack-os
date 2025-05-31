#![no_std]

use core::ptr::null_mut;
use spin::Mutex;

pub const PAGE_SIZE: usize = 4096;
pub const KERNEL_HEAP_START: usize = 0xFFFF_8000_0000_0000;
pub const KERNEL_HEAP_SIZE: usize = 1024 * 1024 * 10; // 10 MiB

static mut HEAP_PTR: usize = KERNEL_HEAP_START;
static MEMORY_MANAGER: Mutex<MemoryManager> = Mutex::new(MemoryManager::new());

pub struct MemoryManager;

impl MemoryManager {
    pub const fn new() -> Self {
        Self
    }

    /// Простая bump-аллокаторная реализация
    pub unsafe fn alloc_kernel(&mut self, size: usize) -> *mut u8 {
        let aligned_size = align_up(size, PAGE_SIZE);
        if HEAP_PTR + aligned_size >= KERNEL_HEAP_START + KERNEL_HEAP_SIZE {
            return null_mut(); // Out of memory
        }

        let ptr = HEAP_PTR as *mut u8;
        HEAP_PTR += aligned_size;
        ptr
    }

    /// Пример инициализации таблиц страниц (упрощённый)
    pub unsafe fn init_page_tables(&mut self) {
        // Здесь будет логика создания таблиц страниц:
        // - создание уровня PML4
        // - связывание виртуальных адресов с физическими
        // - identity mapping (например, 1:1 для ядра)
        // Это псевдокод — в реальной системе здесь будет работа с таблицами.
        log("Page tables initialized (stub)");
    }

    pub fn with_lock<F, R>(f: F) -> R
    where
        F: FnOnce(&mut MemoryManager) -> R,
    {
        let mut mgr = MEMORY_MANAGER.lock();
        f(&mut *mgr)
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

fn log(msg: &str) {
    // Просто заглушка — в реальной ОС заменится на логгер
    let _ = msg;
}
