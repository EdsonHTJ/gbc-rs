use std::sync::{Arc, Mutex, MutexGuard};
use crate::dma::DMA;
use crate::emu::GlobalContext;
use crate::timer::Timer;

#[derive(Debug)]
pub enum TickError {
    GetTicksError,
}

#[derive(Clone)]
pub struct TickManager {
    pub ticks: Arc<Mutex<u64>>,
    pub timer: Arc<Mutex<Timer>>,
    pub dma : Arc<Mutex<DMA>>,
}

impl TickManager {
    pub fn new(timer: Arc<Mutex<Timer>>, global_context: GlobalContext) -> TickManager {
        TickManager {
            ticks: Arc::new(Mutex::new(0)),
            timer,
            dma: global_context.dma.unwrap(),
        }
    }

    pub fn cycle(&self, _cycles: u32) {
        //self.ticks += 1;
        let n = _cycles;
        let mut ticks = self.get_ticks_ref().unwrap();
        for _ in 0..n {
            for _ in 0..4 {
                *ticks += 1;
                self.timer.lock().unwrap().tick();
            }

            self.dma.lock().unwrap().dma_tick();

        }
    }

    fn get_ticks_ref(&self) -> Result<MutexGuard<u64>, TickError> {
        let ticks = match self.ticks.lock() {
            Ok(ticks) => ticks,
            Err(_) => return Err(TickError::GetTicksError),
        };

        Ok(ticks)
    }

    #[allow(dead_code)]
    pub fn increment_ticks(&self) -> Result<(), TickError>{
        let mut ticks = self.get_ticks_ref()?;
        *ticks += 1;
        Ok(())
    }

    pub fn get_ticks(&self) -> Result<u64, TickError> {
        let ticks = self.get_ticks_ref()?;
        Ok(*ticks)
    }

    pub fn set_ticks(&self, new_ticks: u64) -> Result<(), TickError> {
        let mut ticks = self.get_ticks_ref()?;
        *ticks = new_ticks;
        Ok(())
    }
}

