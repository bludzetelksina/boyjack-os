#![no_std]
#![no_main]

use core::arch::asm;

#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    options: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

#[repr(C, align(16))]
struct Idt {
    entries: [IdtEntry; 256],
}

static mut IDT: Idt = Idt {
    entries: [IdtEntry {
        offset_low: 0,
        selector: 0,
        options: 0,
        offset_mid: 0,
        offset_high: 0,
        reserved: 0,
    }; 256],
};

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

extern "C" {
    fn interrupt_handler_stub();
}

/// Установить один дескриптор прерывания
unsafe fn set_idt_entry(n: usize, handler: extern "C" fn()) {
    let handler_addr = handler as u64;
    IDT.entries[n] = IdtEntry {
        offset_low: handler_addr as u16,
        selector: 0x08, // кодовый сегмент
        options: 0x8E00, // present, interrupt gate, ring 0
        offset_mid: (handler_addr >> 16) as u16,
        offset_high: (handler_addr >> 32) as u32,
        reserved: 0,
    };
}

/// Инициализация таблицы прерываний
pub unsafe fn init_idt() {
    set_idt_entry(32, interrupt_handler_stub); // Пример обработчика прерывания таймера (IRQ0)

    let idt_ptr = IdtPointer {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: &IDT as *const _ as u64,
    };

    asm!(
        "lidt [{0}]",
        in(reg) &idt_ptr,
        options(nostack, preserves_flags)
    );
}

/// Включить прерывания
pub unsafe fn enable_interrupts() {
    asm!("sti", options(nomem, nostack));
}

/// Отключить прерывания
pub unsafe fn disable_interrupts() {
    asm!("cli", options(nomem, nostack));
}

/// Простой пример запуска задачи — переключение контекста
/// Здесь заглушка, реально надо реализовать планировщик задач
pub fn switch_task() {
    // TODO: Реализовать переключение контекста
}

/// Выполнение команды hlt для экономии ресурсов CPU
pub fn cpu_halt() {
    unsafe {
        asm!("hlt");
    }
}
