use std::sync::{Arc, Mutex};
use crate::cpu::interrupts::{IFlagsRegister, INTERRUPT_FLAGS};
use crate::dma::DMA;
use crate::emu::GlobalContext;
use crate::io::io_regions::IoRegions;
use crate::lcd::LCD;
use crate::timer::{Timer, TIMER_SINGLETON};

mod io_regions;
mod serial;

#[derive(Debug)]
#[allow(dead_code)]
pub enum IoError {
    InvalidAddress,
}

pub static IO_SINGLETON: Mutex<IO> = Mutex::new(IO{serial_data: 0, serial_control: 0, serial_message: String::new()});

pub struct IO {
    pub serial_data: u8,
    pub serial_control: u8,
    pub serial_message: String,
}

impl IO {
    pub fn new() -> IO {
        IO {
            serial_data: 0,
            serial_control: 0,
            serial_message: String::new(),
        }
    }

    pub fn read(&mut self, address: u8) -> Result<u8, IoError> {
        let io_region = IoRegions::from_u8_address(address)?;
        match io_region {
            IoRegions::SerialTransferData => Ok(self.serial_data),
            IoRegions::SerialTransferControl => Ok(self.serial_control),
            IoRegions::DividerRegister => {
                let ticks = TIMER_SINGLETON.lock().unwrap().get_divider();
                Ok((ticks >> 8) as u8)
            },
            IoRegions::TimerCounter => Ok(TIMER_SINGLETON.lock().unwrap().get_tima()),
            IoRegions::TimerModulo => Ok(TIMER_SINGLETON.lock().unwrap().get_tma()),
            IoRegions::TimerControl => Ok(TIMER_SINGLETON.lock().unwrap().get_tac()),
            IoRegions::InterruptFlags => Ok(INTERRUPT_FLAGS.lock().unwrap().int_flags),
            IoRegions::Lcd => {
                Ok(LCD.lock().unwrap().lcd_read(address as u16))
            },
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
                TIMER_SINGLETON.lock().unwrap().clear_divider();
                Ok(())
            },
            IoRegions::TimerCounter => {
                TIMER_SINGLETON.lock().unwrap().set_tima(data);
                Ok(())
            },
            IoRegions::TimerModulo => {
                TIMER_SINGLETON.lock().unwrap().set_tma(data);
                Ok(())
            },
            IoRegions::TimerControl => {
                TIMER_SINGLETON.lock().unwrap().set_tac(data);
                Ok(())
            },
            IoRegions::InterruptFlags => {
                INTERRUPT_FLAGS.lock().unwrap().int_flags = data;
                Ok(())
            },
            IoRegions::Lcd => {
                LCD.lock().unwrap().lcd_write(address as u16, data);
                Ok(())
            },

            _ => Ok(()),
        }
    }
}