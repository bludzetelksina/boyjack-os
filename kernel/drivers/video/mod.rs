pub mod vga_text;
pub mod framebuffer;

pub enum VideoMode {
    VGA,
    Framebuffer,
}

static mut CURRENT_MODE: VideoMode = VideoMode::VGA;

pub fn init(mode: VideoMode) {
    unsafe {
        CURRENT_MODE = mode;
        match mode {
            VideoMode::VGA => vga_text::clear_screen(),
            VideoMode::Framebuffer => framebuffer::fill_screen(0x000000), // черный экран
        }
    }
}

pub fn print(s: &str) {
    unsafe {
        match CURRENT_MODE {
            VideoMode::VGA => vga_text::print_str(s),
            VideoMode::Framebuffer => {
                // временно ничего
            }
        }
    }
}
