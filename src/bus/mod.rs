use crate::cartridge::{Cartridge, CartridgeError};

/*
    0x0000 - 0x3FFF: 16KB ROM Bank 00 (in cartridge, fixed at bank 00)
    0x4000 - 0x7FFF: 16KB ROM Bank 01..NN (in cartridge, switchable bank number)
    0x8000 - 0x97FF: CHM RAM
    0x9800 - 0x9BFF: BG Map Data 1
    0x9C00 - 0x9FFF: BG Map Data 2
    0xA000 - 0xBFFF: Cartridge RAM
    0xC000 - 0xCFFF: Internal RAM: BANK 0
    0xD000 - 0xDFFF: Internal RAM: BANK 1-7
    0xE000 - 0xFDFF: Echo RAM - Reserved, Do Not Use
    0xFE00 - 0xFE9F: OAM - Object Attribute Memory
    0xFEA0 - 0xFEFF: Unusable Memory
    0xFF00 - 0xFF7F: I/O Ports
    0xFF80 - 0xFFFE: Zero Page
*/

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
        if address < 0x8000 {
            return self.read_from_cartridge(address);
        }

        Err(BusError::NotImplemented)
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        if address < 0x8000 {
            return self.write_to_cartridge(address, data);
        }

        Err(BusError::NotImplemented)
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
