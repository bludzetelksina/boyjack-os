    .section .text
    .global _start

_start:
    // Отключить прерывания
    cpsid i
    cpsid f

    // Настроить стэк (например, указать SP на верхнюю часть доступной памяти)
    ldr sp, =stack_top

    // Очистить регистры (опционально)
    mov r0, #0
    mov r1, #0
    mov r2, #0
    mov r3, #0

    // Передать управление в Rust-функцию ядра (например, kernel_main)
    bl kernel_main

    // Если kernel_main вернётся, зациклиться
hang:
    b hang

// Определение стэка (например, 16 KB)
    .section .bss.stack
    .space 16384
stack_top:
