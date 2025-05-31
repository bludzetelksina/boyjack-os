#![no_std]

use core::ptr;

const VECTOR_TABLE_ADDR: usize = 0x0000_0000;

/// Тип вектора прерываний — указатель на функцию без параметров и возвращаемого значения
type InterruptHandler = extern "C" fn();

/// Вектор прерываний (напр. 8 или 16 записей, зависит от модели)
#[repr(C)]
pub struct VectorTable {
    pub reset: InterruptHandler,
    pub undefined_instruction: InterruptHandler,
    pub software_interrupt: InterruptHandler,
    pub prefetch_abort: InterruptHandler,
    pub data_abort: InterruptHandler,
    pub reserved: InterruptHandler,
    pub irq: InterruptHandler,
    pub fiq: InterruptHandler,
}

/// Заглушечные обработчики прерываний
extern "C" fn default_handler() {
    loop {}
}

/// Инициализация вектора прерываний
pub unsafe fn init_vector_table() {
    let vector_table = VECTOR_TABLE_ADDR as *mut VectorTable;

    ptr::write_volatile(&mut (*vector_table).reset, reset_handler);
    ptr::write_volatile(&mut (*vector_table).undefined_instruction, default_handler);
    ptr::write_volatile(&mut (*vector_table).software_interrupt, default_handler);
    ptr::write_volatile(&mut (*vector_table).prefetch_abort, default_handler);
    ptr::write_volatile(&mut (*vector_table).data_abort, default_handler);
    ptr::write_volatile(&mut (*vector_table).reserved, default_handler);
    ptr::write_volatile(&mut (*vector_table).irq, irq_handler);
    ptr::write_volatile(&mut (*vector_table).fiq, default_handler);
}

/// Пример обработчика reset (сброса)
extern "C" fn reset_handler() {
    // Здесь может быть код инициализации, переход в main ядра
}

/// Пример обработчика IRQ
extern "C" fn irq_handler() {
    // Обработка аппаратного прерывания
}

/// Включить прерывания IRQ
pub unsafe fn enable_irq() {
    asm!("cpsie i", options(nomem, nostack));
}

/// Отключить прерывания IRQ
pub unsafe fn disable_irq() {
    asm!("cpsid i", options(nomem, nostack));
}

/// Простейший запуск задачи — просто вызов функции ядра
pub fn start_task(entry_point: extern "C" fn() -> !) -> ! {
    entry_point()
}
