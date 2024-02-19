use crate::io::io_regions::IoRegions;

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
}

impl IO {
    pub fn new() -> IO {
        IO {
            serial_data: 0,
            serial_control: 0,
            serial_message: String::new(),
        }
    }

    pub fn read(&self, address: u8) -> Result<u8, IoError> {
        let io_region = IoRegions::from_u8_address(address)?;
        match io_region {
            IoRegions::SerialTransferData => Ok(self.serial_data),
            IoRegions::SerialTransferControl => Ok(self.serial_control),
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
            _ => Ok(()),
        }
    }
}