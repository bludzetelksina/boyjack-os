pub mod speaker;
// В будущем: pub mod ac97; pub mod hda;

pub enum AudioDevice {
    PcSpeaker,
    // Ac97,
    // Hda,
}

static mut CURRENT_DEVICE: AudioDevice = AudioDevice::PcSpeaker;

pub fn init(device: AudioDevice) {
    unsafe {
        CURRENT_DEVICE = device;
        match CURRENT_DEVICE {
            AudioDevice::PcSpeaker => {
                // можно протестировать простой beep при инициализации
                speaker::beep(1000);
            }
        }
    }
}

pub fn play_note(freq: u32, duration_ms: u64) {
    unsafe {
        match CURRENT_DEVICE {
            AudioDevice::PcSpeaker => {
                speaker::beep(freq);
                crate::time::sleep_ms(duration_ms); // требуется реализация таймера
                speaker::stop();
            }
        }
    }
}
