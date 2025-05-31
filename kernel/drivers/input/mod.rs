pub mod keyboard;
pub mod mouse;

pub fn init() {
    keyboard::init();
    mouse::init();
}

pub fn handle_input_irq(irq: u8) {
    match irq {
        1 => keyboard::handle_irq(), // PS/2 Keyboard IRQ
        12 => mouse::handle_irq(),   // PS/2 Mouse IRQ
        _ => {}
    }
}
