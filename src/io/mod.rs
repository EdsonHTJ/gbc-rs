use std::sync::{Arc, Mutex};
use crate::cpu::CPU;
use crate::cpu::interrupts::IFlagsRegister;
use crate::io::io_regions::IoRegions;
use crate::timer::Timer;

mod io_regions;
mod serial;

#[derive(Debug)]
#[allow(dead_code)]
pub enum IoError {
    InvalidAddress,
}

pub struct IO {
    pub serial_data: u8,
    pub serial_control: u8,
    pub serial_message: String,
    pub timer: Arc<Mutex<Timer>>,
    pub int_flags: Arc<Mutex<IFlagsRegister>>,
}

impl IO {
    pub fn new(timer: Arc<Mutex<Timer>>, int_flags: Arc<Mutex<IFlagsRegister>>) -> IO {
        IO {
            int_flags,
            serial_data: 0,
            serial_control: 0,
            serial_message: String::new(),
            timer
        }
    }

    pub fn read(&self, address: u8) -> Result<u8, IoError> {
        let io_region = IoRegions::from_u8_address(address)?;
        match io_region {
            IoRegions::SerialTransferData => Ok(self.serial_data),
            IoRegions::SerialTransferControl => Ok(self.serial_control),
            IoRegions::DividerRegister => {
                let ticks = self.timer.lock().unwrap().get_divider();
                Ok((ticks >> 8) as u8)
            },
            IoRegions::TimerCounter => Ok(self.timer.lock().unwrap().get_tima()),
            IoRegions::TimerModulo => Ok(self.timer.lock().unwrap().get_tma()),
            IoRegions::TimerControl => Ok(self.timer.lock().unwrap().get_tac()),
            IoRegions::InterruptFlags => Ok(self.int_flags.lock().unwrap().int_flags),
            _ => Ok(0),
        }

    }

    pub fn write(&mut self, address: u8, data: u8) -> Result<(), IoError>{
        let io_region = IoRegions::from_u8_address(address)?;
        match io_region {
            IoRegions::SerialTransferData => {
                self.serial_data = data;
                Ok(())
            },
            IoRegions::SerialTransferControl => {
                self.serial_control = data;
                self.on_serial_update();
                Ok(())
            },
            IoRegions::DividerRegister => {
                self.timer.lock().unwrap().clear_divider();
                Ok(())
            },
            IoRegions::TimerCounter => {
                self.timer.lock().unwrap().set_tima(data);
                Ok(())
            },
            IoRegions::TimerModulo => {
                self.timer.lock().unwrap().set_tma(data);
                Ok(())
            },
            IoRegions::TimerControl => {
                self.timer.lock().unwrap().set_tac(data);
                Ok(())
            },
            IoRegions::InterruptFlags => {
                self.int_flags.lock().unwrap().int_flags = data;
                Ok(())
            },

            _ => Ok(()),
        }
    }
}