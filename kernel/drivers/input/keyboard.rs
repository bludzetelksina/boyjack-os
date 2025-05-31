use x86::io::{inb, outb};
use spin::Mutex;
use lazy_static::lazy_static;

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;

lazy_static! {
    static ref LAST_KEY: Mutex<Option<u8>> = Mutex::new(None);
}

pub fn init() {
    println!("[keyboard] PS/2 клавиатура инициализирована");
}

pub fn handle_irq() {
    let scancode = unsafe { inb(PS2_DATA_PORT) };
    *LAST_KEY.lock() = Some(scancode);
    println!("[keyboard] Скан-код: {:#x}", scancode);
}

pub fn read_scancode() -> Option<u8> {
    LAST_KEY.lock().take()
}
