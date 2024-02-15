use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub enum TickError {
    GetTicksError,
}

#[derive(Clone)]
pub struct TickManager {
    pub ticks: Arc<Mutex<u64>>,
}

impl TickManager {
    pub fn new() -> TickManager {
        TickManager {
            ticks: Arc::new(Mutex::new(0)),
        }
    }

    pub fn cycle(&self, _cycles: u32) {
        //self.ticks += 1;
    }

    fn get_ticks_ref(&self) -> Result<MutexGuard<u64>, TickError> {
        let ticks = match self.ticks.lock() {
            Ok(ticks) => ticks,
            Err(_) => return Err(TickError::GetTicksError),
        };

        Ok(ticks)
    }

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

