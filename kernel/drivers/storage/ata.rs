use x86::io::{inb, inw, outb, outw};
use core::ptr;

const ATA_PRIMARY_IO: u16 = 0x1F0;
const ATA_PRIMARY_CTRL: u16 = 0x3F6;

const ATA_CMD_READ_SECTORS: u8 = 0x20;

pub fn init() {
    println!("[storage] ATA PIO драйвер инициализирован");
}

pub fn read_sector(lba: u32, buffer: &mut [u8]) {
    assert!(buffer.len() >= 512);

    unsafe {
        // Подготовка устройства: выбор диска (Master)
        outb(ATA_PRIMARY_IO + 6, 0xE0 | ((lba >> 24) & 0x0F) as u8);
        outb(ATA_PRIMARY_IO + 1, 0x00); // disable interrupts
        outb(ATA_PRIMARY_IO + 2, 0x01); // count: 1 sector
        outb(ATA_PRIMARY_IO + 3, (lba & 0xFF) as u8);
        outb(ATA_PRIMARY_IO + 4, ((lba >> 8) & 0xFF) as u8);
        outb(ATA_PRIMARY_IO + 5, ((lba >> 16) & 0xFF) as u8);
        outb(ATA_PRIMARY_IO + 7, ATA_CMD_READ_SECTORS);

        wait_bsy();

        for i in 0..256 {
            let word = inw(ATA_PRIMARY_IO);
            let bytes = word.to_le_bytes();
            buffer[i * 2] = bytes[0];
            buffer[i * 2 + 1] = bytes[1];
        }

        println!("[storage] Прочитан сектор LBA {}", lba);
    }
}

unsafe fn wait_bsy() {
    while inb(ATA_PRIMARY_IO + 7) & 0x80 != 0 {}
    while inb(ATA_PRIMARY_IO + 7) & 0x08 == 0 {}
}
