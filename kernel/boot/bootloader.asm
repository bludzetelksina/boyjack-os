; kernel/boot/bootloader.asm
; Минимальный загрузчик для x86, переключение в Protected Mode

[BITS 16]
[ORG 0x7C00]          ; Адрес загрузки BIOS

start:
    cli               ; Отключить прерывания

    ; Инициализация сегментов
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00    ; Установить стек

    ; Загрузить GDT (глобальная таблица дескрипторов)
    lgdt [gdt_descriptor]

    ; Включить Protected Mode (PE bit в CR0)
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; Переключиться в Protected Mode с помощью прыжка
    jmp CODE_SEG:init_pm

[BITS 32]
init_pm:
    ; Установить сегменты для Protected Mode
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Инициализировать стек в Protected Mode
    mov esp, 0x9FC00

    ; Переход к следующему этапу (например, загрузке ядра на 0x100000)
    call load_kernel

hang:
    hlt
    jmp hang

load_kernel:
    ; Здесь можно добавить загрузку ядра в память,
    ; например, с диска через BIOS прерывания.
    ; Для минимального примера просто переходим по адресу 0x100000

    mov eax, 0x100000
    jmp eax

; GDT: простой набор сегментов для Protected Mode
gdt_start:
    ; NULL descriptor
    dd 0x0
    dd 0x0

    ; Кодовый сегмент (base=0, limit=4GB, executable, readable)
    dd 0x0000FFFF
    dd 0x00CF9A00

    ; Данные сегмент (base=0, limit=4GB, writable)
    dd 0x0000FFFF
    dd 0x00CF9200

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1  ; Limit (size of GDT - 1)
    dd gdt_start                ; Base (address of GDT)

; Сегментные селекторы для GDT
CODE_SEG equ 0x08
DATA_SEG equ 0x10

; Заполнитель до 512 байт
times 510-($-$$) db 0
dw 0xAA55       ; Загрузочная сигнатура
