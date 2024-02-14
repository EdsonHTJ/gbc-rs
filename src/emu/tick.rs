use std::sync::{Arc, Mutex};

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

    pub fn cycle(&self) {
        //self.ticks += 1;
    }

    pub fn increment_ticks(&self) {
        let mut ticks = self.ticks.lock().unwrap();
        *ticks += 1;
    }

    pub fn get_ticks(&self) -> u64 {
        let ticks = self.ticks.lock().unwrap();
        *ticks
    }
}

