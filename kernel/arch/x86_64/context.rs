#![no_std]
#![no_main]

use core::arch::asm;

/// Структура, представляющая сохранённый контекст процессора (регистры общего назначения + RIP, RSP)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
}

/// Сохраняет текущий контекст в структуру и загружает контекст следующей задачи
/// 
/// # Safety
/// Использует inline asm, нужно быть осторожным с регистрами
#[naked]
pub unsafe extern "C" fn switch_context(old: *mut CpuContext, new: *const CpuContext) {
    asm!(
        // Сохраняем регистры в old
        "mov [rdi + 0x00], r15",
        "mov [rdi + 0x08], r14",
        "mov [rdi + 0x10], r13",
        "mov [rdi + 0x18], r12",
        "mov [rdi + 0x20], r11",
        "mov [rdi + 0x28], r10",
        "mov [rdi + 0x30], r9",
        "mov [rdi + 0x38], r8",
        "mov [rdi + 0x40], rsi",
        "mov [rdi + 0x48], rdi",
        "mov [rdi + 0x50], rbp",
        "mov [rdi + 0x58], rdx",
        "mov [rdi + 0x60], rcx",
        "mov [rdi + 0x68], rbx",
        "mov [rdi + 0x70], rax",

        // Сохраняем rsp и rflags
        "mov rax, rsp",
        "mov [rdi + 0x80], rax",
        "pushfq",
        "pop qword ptr [rdi + 0x88]",

        // Загружаем регистры из new
        "mov r15, [rsi + 0x00]",
        "mov r14, [rsi + 0x08]",
        "mov r13, [rsi + 0x10]",
        "mov r12, [rsi + 0x18]",
        "mov r11, [rsi + 0x20]",
        "mov r10, [rsi + 0x28]",
        "mov r9,  [rsi + 0x30]",
        "mov r8,  [rsi + 0x38]",
        "mov rsi, [rsi + 0x40]",
        "mov rdi, [rsi + 0x48]",
        "mov rbp, [rsi + 0x50]",
        "mov rdx, [rsi + 0x58]",
        "mov rcx, [rsi + 0x60]",
        "mov rbx, [rsi + 0x68]",
        "mov rax, [rsi + 0x70]",

        // Восстанавливаем rsp и rflags
        "mov rsp, [rsi + 0x80]",
        "push qword ptr [rsi + 0x88]",
        "popfq",

        // Переходим по адресу rip
        "mov rax, [rsi + 0x78]",
        "jmp rax",
        options(noreturn)
    );
}
