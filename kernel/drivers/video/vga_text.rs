const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

pub fn clear_screen() {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            write_char(x, y, ' ', 0x07);
        }
    }
}

pub fn write_char(x: usize, y: usize, c: char, color: u8) {
    let offset = 2 * (y * WIDTH + x);
    unsafe {
        VGA_BUFFER.add(offset).write_volatile(c as u8);
        VGA_BUFFER.add(offset + 1).write_volatile(color);
    }
}

pub fn print_str(s: &str) {
    static mut CURSOR_X: usize = 0;
    static mut CURSOR_Y: usize = 0;

    for byte in s.bytes() {
        unsafe {
            match byte {
                b'\n' => {
                    CURSOR_X = 0;
                    CURSOR_Y += 1;
                }
                _ => {
                    write_char(CURSOR_X, CURSOR_Y, byte as char, 0x0F);
                    CURSOR_X += 1;
                    if CURSOR_X >= WIDTH {
                        CURSOR_X = 0;
                        CURSOR_Y += 1;
                    }
                }
            }
        }
    }
}
