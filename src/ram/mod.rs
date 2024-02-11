
#[derive(Debug)]
pub enum RamError {
    InvalidAddress,
}
pub struct Ram {
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x80],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            wram: [0; 0x2000],
            hram: [0; 0x80],
        }
    }

    pub fn write_wram(&mut self, address: u16, data: u8) -> Result<(), RamError> {
        if address as usize > self.wram.len() {
            return Err(RamError::InvalidAddress);
        }

        self.wram[address as usize] = data;
        Ok(())
    }

    pub fn read_wram(&self, address: u16) -> Result<u8, RamError> {
        if address as usize > self.wram.len() {
            return Err(RamError::InvalidAddress);
        }

        Ok(self.wram[address as usize])
    }

    pub fn write_hram(&mut self, address: u16, data: u8) -> Result<(), RamError> {
        if address as usize > self.hram.len() {
            return Err(RamError::InvalidAddress);
        }

        self.hram[address as usize] = data;
        Ok(())
    }

    pub fn read_hram(&self, address: u16) -> Result<u8, RamError> {
        if address as usize > self.hram.len() {
            return Err(RamError::InvalidAddress);
        }

        Ok(self.hram[address as usize])
    }
}