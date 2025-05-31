/// Использует порт 0x61 и 0x43/0x42 для управления динамиком и таймером

const PIT_FREQUENCY: u32 = 1193180;

pub fn beep(frequency: u32) {
    let divisor = PIT_FREQUENCY / frequency;

    unsafe {
        // Устанавливаем режим Square Wave Generator (режим 3)
        x86::io::outb(0x43, 0b10110110);

        // Устанавливаем делитель
        x86::io::outb(0x42, (divisor & 0xFF) as u8);
        x86::io::outb(0x42, (divisor >> 8) as u8);

        // Включаем динамик (бит 0 и 1)
        let tmp = x86::io::inb(0x61);
        if tmp & 3 != 3 {
            x86::io::outb(0x61, tmp | 3);
        }
    }
}

pub fn stop() {
    unsafe {
        let tmp = x86::io::inb(0x61) & 0xFC;
        x86::io::outb(0x61, tmp);
    }
}
