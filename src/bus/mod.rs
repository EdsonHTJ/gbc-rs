mod addresses;
mod writers;

use crate::bus::addresses::AddrSpace;
use crate::cartridge::{Cartridge, CartridgeError};

#[derive(Debug)]
pub enum BusError {
    NotImplemented,
    NoCartridgeLoaded,
    CartridgeError(CartridgeError),
}

pub struct BUS {
    pub cartridge: Option<Cartridge>,
}

impl BUS {
    pub fn new() -> BUS {
        BUS { cartridge: None }
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
}
