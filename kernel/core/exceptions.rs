use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use lazy_static::lazy_static;

lazy_static! {
    static ref EXCEPTION_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        idt
    };
}

/// Инициализация обработчиков исключений
pub fn init_exceptions() {
    EXCEPTION_IDT.load();
}

/// Деление на ноль
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    crate::drivers::tty::print_str("EXCEPTION: Divide by zero\n");
    crate::drivers::tty::print_stack_frame(&stack_frame);
    halt();
}

/// Недопустимая инструкция
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    crate::drivers::tty::print_str("EXCEPTION: Invalid Opcode\n");
    crate::drivers::tty::print_stack_frame(&stack_frame);
    halt();
}

/// Ошибка общей защиты
extern "x86-interrupt" fn general_protection_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    crate::drivers::tty::print_str("EXCEPTION: General Protection Fault\n");
    crate::drivers::tty::print_str(&format!("Error Code: {}\n", error_code));
    crate::drivers::tty::print_stack_frame(&stack_frame);
    halt();
}

/// Двойное исключение (например, ошибка в обработчике)
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    crate::drivers::tty::print_str("EXCEPTION: Double Fault\n");
    crate::drivers::tty::print_stack_frame(&stack_frame);
    panic!("System halted due to double fault.");
}

/// Остановка системы после исключения
fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
