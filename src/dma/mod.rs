use std::sync::{Arc, Mutex};
use crate::bus::{BUS, BusMutex};
use crate::emu::GlobalContext;
use crate::ppu::PPU;

pub struct DMA {
    active: bool,
    byte: u8,
    value: u8,
    start_delay: u8,
    ppu: Option<Arc<Mutex<PPU>>>,
    bus : Option<BusMutex>,
}

impl DMA {
    pub fn new(global_context: GlobalContext) -> DMA {
        DMA {
            active: false,
            byte: 0,
            value: 0,
            start_delay: 0,
            ppu: global_context.ppu,
            bus: global_context.bus,
        }
    }

    pub fn attach_bus(&mut self, bus: BusMutex) {
        self.bus = Some(bus);
    }

    pub fn attach_ppu(&mut self, ppu: Arc<Mutex<PPU>>) {
        self.ppu = Some(ppu);
    }

    pub fn dma_start(&mut self, value: u8) {
        self.active = true;
        self.value = value;
        self.byte = 0;
        self.start_delay = 2;
    }

    pub fn dma_tick(&mut self) {
        if !self.active {
            return;
        }

        if self.start_delay > 0 {
            self.start_delay -= 1;
            return;
        }

        if self.active {
            self.byte += 1;
            if self.byte == 0 {
                self.active = false;
            }
        }

        self.ppu.as_mut().unwrap().lock().unwrap().oam_write(self.byte as u16, self.bus.as_mut().unwrap().read((self.value as u16) * 0x100 + self.byte as u16).unwrap());
        self.byte += 1;
        self.active = self.byte < 0xA0;
    }

    pub fn dma_transferring(&self) -> bool {
        self.active
    }
}