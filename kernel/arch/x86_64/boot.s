.section .text
.global _start
_start:
    cli                     # Отключить прерывания

    # Загрузить сегменты в регистры сегментов
    xor %ax, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %ss
    mov %ax, %fs
    mov %ax, %gs

    # Загрузка GDT (Global Descriptor Table)
    lea gdt_descriptor(%rip), %rax
    lgdt (%rax)

    # Включить режим Protected Mode (если мы ещё не в нем)
    # Для простоты считаем, что GRUB уже загрузил в Protected Mode с включённым Long Mode

    # Установить сегментный регистр кодового сегмента для 64-битного режима
    mov $0x08, %ax          # селектор кодового сегмента в GDT
    mov %ax, %cs
    # Переход в 64-битный режим (long mode)
    # На самом деле, переход выполняется через far jump:
    jmpq $0x08, $kernel_main

.section .data
gdt_start:
    .quad 0x0000000000000000  # NULL descriptor
    .quad 0x00AF9A000000FFFF  # Кодовый сегмент 64-bit
    .quad 0x00AF92000000FFFF  # Данные сегмент
gdt_end:

gdt_descriptor:
    .word gdt_end - gdt_start - 1  # size
    .quad gdt_start                # адрес

.section .text
kernel_main:
    # Тут начинается Rust код, вызываемый из ASM
    # Передача управления на функцию ядра (в Rust)
    extern kernel_main_rust
    call kernel_main_rust

    # Если kernel_main_rust вернёт управление, бесконечный цикл
.hang:
    hlt
    jmp .hang
