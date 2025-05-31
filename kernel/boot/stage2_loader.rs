#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod memory;
mod interrupts;
mod kernel_main;

#[no_mangle]
pub extern "C" fn stage2_entry() -> ! {
    // Инициализация памяти (страничная таблица, менеджер памяти)
    memory::init();

    // Инициализация прерываний
    interrupts::init();

    // Запуск главной функции ядра
    kernel_main::start_kernel();

    // Если ядро завершилось (не должно), зацикливаемся
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

// Обработчик паники
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
