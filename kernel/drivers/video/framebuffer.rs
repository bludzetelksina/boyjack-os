use core::ptr::write_volatile;

static mut FRAMEBUFFER_ADDR: usize = 0;
static mut WIDTH: usize = 1024;
static mut HEIGHT: usize = 768;
static mut PITCH: usize = 4096;

#[repr(C)]
pub struct FramebufferInfo {
    pub address: usize,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

pub fn init(info: FramebufferInfo) {
    unsafe {
        FRAMEBUFFER_ADDR = info.address;
        WIDTH = info.width;
        HEIGHT = info.height;
        PITCH = info.pitch;
    }
}

pub fn draw_pixel(x: usize, y: usize, color: u32) {
    unsafe {
        let offset = FRAMEBUFFER_ADDR + y * PITCH + x * 4;
        let pixel_ptr = offset as *mut u32;
        write_volatile(pixel_ptr, color);
    }
}

pub fn fill_screen(color: u32) {
    for y in 0..unsafe { HEIGHT } {
        for x in 0..unsafe { WIDTH } {
            draw_pixel(x, y, color);
        }
    }
}
