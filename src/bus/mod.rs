mod addresses;
mod writers;

use crate::bus::addresses::AddrSpace;
use crate::cartridge::{Cartridge, CartridgeError};
use crate::ram::{Ram, RamError};

#[derive(Debug)]
pub enum BusError {
    NotImplemented,
    NoCartridgeLoaded,
    CartridgeError(CartridgeError),
    InvalidAddress,
    RamError(RamError),
}

impl From<RamError> for BusError {
    fn from(e: RamError) -> BusError {
        BusError::RamError(e)
    }
}

pub struct BUS {
    cartridge: Option<Cartridge>,
    ram: Ram,
    interrupt_register: u8,
}

impl BUS {
    pub fn new() -> BUS {
        BUS {
            cartridge: None,
            ram: Ram::new(),
            interrupt_register: 0,
        }
    }

    pub fn load_game(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        let cartridge = Cartridge::new(rom)?;
        self.cartridge = Some(cartridge);
        Ok(())
    }

    pub fn read(&mut self, address: u16) -> Result<u8, BusError> {
        let region = AddrSpace::from_address(&address)?;
        let reader = writers::get_writer_by_region(region)?;
        reader.read(self, address)
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let region = AddrSpace::from_address(&address)?;
        let writer = writers::get_writer_by_region(region)?;
        writer.write(self, address, data)
    }

    pub fn write_16(&mut self, address: u16, data: u16) -> Result<(), BusError> {
        self.write(address, (data & 0xFF) as u8)?;
        self.write(address + 1, (data >> 8) as u8)?;
        Ok(())
    }

    pub fn read_16(&mut self, address: u16) -> Result<u16, BusError> {
        let low = self.read(address)? as u16;
        let high = self.read(address + 1)? as u16;
        Ok((high << 8) | low)
    }

    fn read_from_cartridge(&mut self, address: u16) -> Result<u8, BusError> {
        if self.cartridge.is_none() {
            return Err(BusError::NoCartridgeLoaded);
        }

        let cartridge = self.cartridge.as_mut().unwrap();

        cartridge
            .read(address)
            .map_err(|e| BusError::CartridgeError(e))
    }

    fn write_to_cartridge(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        if self.cartridge.is_none() {
            return Err(BusError::NoCartridgeLoaded);
        }

        let cartridge = self.cartridge.as_mut().unwrap();

        cartridge
            .write(address, data)
            .map_err(|e| BusError::CartridgeError(e))
    }

    fn read_from_ram(&self, address: u16) -> Result<u8, BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_ram_offset(address)?;
        match region {
            AddrSpace::RAM0 | AddrSpace::RAM1 => Ok(self.ram.read_wram(address)?),
            AddrSpace::ZP => Ok(self.ram.read_hram(address)?),
            _ => Err(BusError::InvalidAddress),
        }
    }

    fn write_to_ram(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_ram_offset(address)?;
        match region {
            AddrSpace::RAM0 | AddrSpace::RAM1 => self.ram.write_wram(address, data)?,
            AddrSpace::ZP => self.ram.write_hram(address, data)?,
            _ => return Err(BusError::InvalidAddress),
        }

        Ok(())
    }

    fn read_from_master_interruption_register(&self) -> u8 {
        self.interrupt_register
    }

    fn write_to_master_interruption_register(&mut self, data: u8) {
        self.interrupt_register = data;
    }
}
