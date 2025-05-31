#![no_std]

use core::arch::asm;

/// Структура для хранения регистров процессора ARMv7 при переключении задач
#[repr(C)]
pub struct CPUContext {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,   // Stack Pointer
    pub lr: u32,   // Link Register
    pub pc: u32,   // Program Counter
    pub cpsr: u32, // Current Program Status Register
}

impl CPUContext {
    /// Создание пустого контекста (например, для новой задачи)
    pub const fn new() -> Self {
        CPUContext {
            r0: 0, r1: 0, r2: 0, r3: 0,
            r4: 0, r5: 0, r6: 0, r7: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, sp: 0, lr: 0, pc: 0,
            cpsr: 0x60000010, // User mode, interrupts enabled (пример)
        }
    }
}

/// Переключение контекста между двумя задачами
///
/// # Safety
///
/// Требует корректных указателей на контексты.
///
#[naked]
pub unsafe extern "C" fn context_switch(old_ctx: *mut CPUContext, new_ctx: *const CPUContext) {
    asm!(
        // Сохранить регистры r4-r12, sp, lr в old_ctx
        "stmia {0}!, {{r4-r12}}",
        "str sp, [{0}, #36]",
        "str lr, [{0}, #40]",
        // Сохранить pc и cpsr (здесь pc будет адресом инструкции после bl, взять из lr)
        "mrs r1, cpsr",
        "str r1, [{0}, #44]",
        "ldr r1, [lr, #-4]",  // Получить адрес вызова
        "str r1, [{0}, #48]",

        // Загрузить регистры r4-r12, sp, lr из new_ctx
        "ldmia {1}!, {{r4-r12}}",
        "ldr sp, [{1}, #36]",
        "ldr lr, [{1}, #40]",
        // Загрузить pc и cpsr
        "ldr r1, [{1}, #44]",
        "msr cpsr, r1",
        "ldr r1, [{1}, #48]",
        "mov pc, r1",

        in(reg) old_ctx,
        in(reg) new_ctx,
        options(noreturn)
    );
}
