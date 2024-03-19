use std::sync::{Arc, Mutex};
use crate::cpu::CPU;
use crate::cpu::interrupts::{IFlagsRegister, InterruptType};

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    int_flags: Arc<Mutex<IFlagsRegister>>
}

impl Timer {
    pub fn new(int_flags: Arc<Mutex<IFlagsRegister>>) -> Self {
        Timer {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            int_flags
        }
    }

    pub fn reset(&mut self) {
        let mut new_timer = Timer::new(self.int_flags.clone());
        std::mem::swap(self, &mut new_timer);
    }

    pub fn tick(&mut self) {
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);

        let mut timer_update = false;
        match self.tac & 0x3 {
            0 => {
                if (prev_div & (1 << 9)) != (self.div & (1 << 9)) {
                    timer_update = true;
                }
            }
            1 => {
                if (prev_div & (1 << 3)) != (self.div & (1 << 3)) {
                    timer_update = true;
                }
            }
            2 => {
                if (prev_div & (1 << 5)) != (self.div & (1 << 5)) {
                    timer_update = true;
                }
            }
            3 => {
                if (prev_div & (1 << 7)) != (self.div & (1 << 7)) {
                    timer_update = true;
                }
            }
            _ => {}
        }

        if timer_update && ((self.tac & (1 << 2)) != 0){
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0xFF {
                self.tima = self.tma;
                {
                    self.int_flags.lock().unwrap().add_interrupt(InterruptType::Timer);
                }
            }
        }
    }

    pub fn clear_divider(&mut self) {
        self.div = 0;
    }

    pub fn set_tima(&mut self, data: u8) {
        self.tima = data;
    }

    pub fn set_tma(&mut self, data: u8) {
        self.tma = data;
    }

    pub fn set_tac(&mut self, data: u8) {
        self.tac = data;
    }

    pub fn get_divider(&self) -> u16 {
        self.div
    }

    pub fn get_tima(&self) -> u8 {
        self.tima
    }

    pub fn get_tma(&self) -> u8 {
        self.tma
    }

    pub fn get_tac(&self) -> u8 {
        self.tac
    }
}