use core::ptr::{read_volatile, write_volatile};
use crate::pci;
use crate::interrupts;

const E1000_REG_CTRL: usize = 0x0000;
const E1000_REG_STATUS: usize = 0x0008;
const E1000_REG_RCTL: usize = 0x0100;
const E1000_REG_TCTL: usize = 0x0400;
const E1000_REG_RDBAL: usize = 0x2800;
const E1000_REG_RDBAH: usize = 0x2804;
const E1000_REG_RDLEN: usize = 0x2808;
const E1000_REG_RDH: usize = 0x2810;
const E1000_REG_RDT: usize = 0x2818;
const E1000_REG_TDBAL: usize = 0x3800;
const E1000_REG_TDBAH: usize = 0x3804;
const E1000_REG_TDLEN: usize = 0x3808;
const E1000_REG_TDH: usize = 0x3810;
const E1000_REG_TDT: usize = 0x3818;

const RX_DESC_COUNT: usize = 32;
const TX_DESC_COUNT: usize = 8;
const ETHERNET_FRAME_MAX_SIZE: usize = 1518;

#[repr(C, packed)]
struct RxDesc {
    addr: u64,
    length: u16,
    checksum: u16,
    status: u8,
    errors: u8,
    special: u16,
}

#[repr(C, packed)]
struct TxDesc {
    addr: u64,
    length: u16,
    cso: u8,
    cmd: u8,
    status: u8,
    css: u8,
    special: u16,
}

pub struct E1000 {
    mmio_base: *mut u8,
    rx_descs: [RxDesc; RX_DESC_COUNT],
    tx_descs: [TxDesc; TX_DESC_COUNT],
    rx_buffers: [[u8; ETHERNET_FRAME_MAX_SIZE]; RX_DESC_COUNT],
    tx_buffers: [[u8; ETHERNET_FRAME_MAX_SIZE]; TX_DESC_COUNT],
    rx_tail: usize,
    tx_tail: usize,
}

impl E1000 {
    pub fn new(pci_device: &pci::Device) -> Option<Self> {
        let mmio_base = pci_device.bar(0)? as *mut u8;

        let mut nic = E1000 {
            mmio_base,
            rx_descs: unsafe { core::mem::zeroed() },
            tx_descs: unsafe { core::mem::zeroed() },
            rx_buffers: [[0; ETHERNET_FRAME_MAX_SIZE]; RX_DESC_COUNT],
            tx_buffers: [[0; ETHERNET_FRAME_MAX_SIZE]; TX_DESC_COUNT],
            rx_tail: RX_DESC_COUNT - 1,
            tx_tail: 0,
        };

        nic.init_rx();
        nic.init_tx();
        nic.enable();

        Some(nic)
    }

    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { read_volatile(self.mmio_base.add(offset) as *const u32) }
    }

    fn write_reg(&self, offset: usize, value: u32) {
        unsafe { write_volatile(self.mmio_base.add(offset) as *mut u32, value) }
    }

    fn init_rx(&mut self) {
        // Настройка дескрипторов RX и буферов
        for i in 0..RX_DESC_COUNT {
            let buf_ptr = &self.rx_buffers[i] as *const _ as u64;
            self.rx_descs[i].addr = buf_ptr;
            self.rx_descs[i].status = 0;
        }
        // Установка базового адреса, длины, и индексов
        let rx_desc_ptr = &self.rx_descs as *const _ as u64;
        self.write_reg(E1000_REG_RDBAL, rx_desc_ptr as u32);
        self.write_reg(E1000_REG_RDBAH, (rx_desc_ptr >> 32) as u32);
        self.write_reg(E1000_REG_RDLEN, (RX_DESC_COUNT * core::mem::size_of::<RxDesc>()) as u32);
        self.write_reg(E1000_REG_RDH, 0);
        self.write_reg(E1000_REG_RDT, RX_DESC_COUNT as u32 - 1);

        // Включение приёма пакетов (RCTL)
        let rctl = 0x00000002 | 0x00000010 | 0x00000040; // пример настроек
        self.write_reg(E1000_REG_RCTL, rctl);
    }

    fn init_tx(&mut self) {
        for i in 0..TX_DESC_COUNT {
            let buf_ptr = &self.tx_buffers[i] as *const _ as u64;
            self.tx_descs[i].addr = buf_ptr;
            self.tx_descs[i].status = 0x1; // готово к передаче
        }
        let tx_desc_ptr = &self.tx_descs as *const _ as u64;
        self.write_reg(E1000_REG_TDBAL, tx_desc_ptr as u32);
        self.write_reg(E1000_REG_TDBAH, (tx_desc_ptr >> 32) as u32);
        self.write_reg(E1000_REG_TDLEN, (TX_DESC_COUNT * core::mem::size_of::<TxDesc>()) as u32);
        self.write_reg(E1000_REG_TDH, 0);
        self.write_reg(E1000_REG_TDT, 0);

        // Включение передачи (TCTL)
        let tctl = 0x00000002 | 0x00000008 | 0x00000100; // пример настроек
        self.write_reg(E1000_REG_TCTL, tctl);
    }

    fn enable(&self) {
        let ctrl = self.read_reg(E1000_REG_CTRL);
        self.write_reg(E1000_REG_CTRL, ctrl | 0x40); // включить устройство
    }

    pub fn handle_irq(&mut self) {
        // Обработка приёма пакетов (упрощённо)
        let mut idx = self.rx_tail;
        loop {
            let status = self.rx_descs[idx].status;
            if status & 0x01 == 0 {
                break; // пакет не готов
            }

            let length = self.rx_descs[idx].length as usize;
            let packet = &self.rx_buffers[idx][..length];

            println!("[network] Получен пакет, длина: {}", length);
            // Здесь можно передать пакет дальше в стек протоколов

            self.rx_descs[idx].status = 0; // освободить дескриптор
            self.rx_tail = (idx + 1) % RX_DESC_COUNT;
            idx = self.rx_tail;
            self.write_reg(E1000_REG_RDT, self.rx_tail as u32);
        }
    }
}
