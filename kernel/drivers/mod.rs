pub mod video;
pub mod audio;
pub mod input;
pub mod storage;
pub mod network;

// Структура для хранения состояния драйверов, если нужно
pub struct Drivers {
    pub video: Option<video::VideoDriver>,
    pub audio: Option<audio::AudioDriver>,
    pub input: Option<input::InputDriver>,
    pub storage: Option<storage::StorageDriver>,
    pub network: Option<network::NetworkDriver>,
}

impl Drivers {
    pub fn new() -> Self {
        Self {
            video: None,
            audio: None,
            input: None,
            storage: None,
            network: None,
        }
    }

    // Инициализация всех драйверов — пример последовательного вызова init
    pub fn init_all(&mut self) {
        // Инициализация видео драйвера
        self.video = Some(video::VideoDriver::init());

        // Инициализация аудио драйвера
        self.audio = Some(audio::AudioDriver::init());

        // Инициализация устройств ввода
        self.input = Some(input::InputDriver::init());

        // Инициализация драйвера хранения данных
        self.storage = Some(storage::StorageDriver::init());

        // Инициализация сетевого драйвера
        self.network = Some(network::NetworkDriver::init());
    }
}

// Функция регистрации драйверов в ядро (вызывается из main или init ядра)
pub fn register_drivers() -> Drivers {
    let mut drivers = Drivers::new();
    drivers.init_all();
    drivers
}
