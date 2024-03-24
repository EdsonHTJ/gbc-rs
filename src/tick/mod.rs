use std::sync::{Arc, Mutex, MutexGuard};
use crate::dma::DMA;
use crate::emu::GlobalContext;
use crate::timer::{Timer, TIMER_SINGLETON};

pub static TICKER_SINGLETON: Mutex<TickManager> = Mutex::new(TickManager{ticks: 0});

#[derive(Clone)]
pub struct TickManager {
    pub ticks: u64,
}

impl TickManager {
    pub fn new() -> TickManager {
        TickManager {
            ticks: 0,
        }
    }

    pub fn cycle(&mut self, _cycles: u32) {
        //self.ticks += 1;
        let n = _cycles;
        for _ in 0..n {
            for _ in 0..4 {
                self.increment_ticks();
                TIMER_SINGLETON.lock().unwrap().tick();
            }

            DMA.lock().unwrap().dma_tick();

        }
    }


    #[allow(dead_code)]
    pub fn increment_ticks(&mut self) {
        self.ticks += 1;
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn set_ticks(&mut self, new_ticks: u64) {
        self.ticks = new_ticks;
    }
}

