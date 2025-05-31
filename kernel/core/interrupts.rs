use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use crate::drivers::{keyboard::handle_keypress, timer::tick};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

/// Инициализация IDT (таблицы дескрипторов прерываний)
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::drivers::tty::print_str("EXCEPTION: Breakpoint\n");
    crate::drivers::tty::print_stack_frame(&stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    let fault_addr = Cr2::read();
    crate::drivers::tty::print_str("EXCEPTION: Page Fault\n");
    crate::drivers::tty::print_str(&format!("Accessed address: {:?}\n", fault_addr));
    crate::drivers::tty::print_str(&format!("Error code: {:?}\n", error_code));
    crate::drivers::tty::print_stack_frame(&stack_frame);

    loop {}
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    tick();
    unsafe { crate::arch::x86_64::pic::PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    handle_keypress();
    unsafe { crate::arch::x86_64::pic::PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); }
}

/// Индексы IRQ прерываний
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
