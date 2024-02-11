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

use crate::bus::BusError;

pub enum AddrSpace {
    ROM0,
    ROM1,
    CHM,
    BG1,
    BG2,
    CRAM,
    RAM0,
    RAM1,
    ECHO,
    OAM,
    UNUSABLE,
    IO,
    ZP,
}

impl AddrSpace {
    pub fn get_region(&self) -> (u16, u16) {
        match self {
            AddrSpace::ROM0 => (0x0000, 0x3FFF),
            AddrSpace::ROM1 => (0x4000, 0x7FFF),
            AddrSpace::CHM => (0x8000, 0x97FF),
            AddrSpace::BG1 => (0x9800, 0x9BFF),
            AddrSpace::BG2 => (0x9C00, 0x9FFF),
            AddrSpace::CRAM => (0xA000, 0xBFFF),
            AddrSpace::RAM0 => (0xC000, 0xCFFF),
            AddrSpace::RAM1 => (0xD000, 0xDFFF),
            AddrSpace::ECHO => (0xE000, 0xFDFF),
            AddrSpace::OAM => (0xFE00, 0xFE9F),
            AddrSpace::UNUSABLE => (0xFEA0, 0xFEFF),
            AddrSpace::IO => (0xFF00, 0xFF7F),
            AddrSpace::ZP => (0xFF80, 0xFFFE),
        }
    }

    pub fn from_address(address: &u16) -> Result<Self, BusError> {
        let region = match address {
            0x0000..=0x3FFF => AddrSpace::ROM0,
            0x4000..=0x7FFF => AddrSpace::ROM1,
            0x8000..=0x97FF => AddrSpace::CHM,
            0x9800..=0x9BFF => AddrSpace::BG1,
            0x9C00..=0x9FFF => AddrSpace::BG2,
            0xA000..=0xBFFF => AddrSpace::CRAM,
            0xC000..=0xCFFF => AddrSpace::RAM0,
            0xD000..=0xDFFF => AddrSpace::RAM1,
            0xE000..=0xFDFF => AddrSpace::ECHO,
            0xFE00..=0xFE9F => AddrSpace::OAM,
            0xFEA0..=0xFEFF => AddrSpace::UNUSABLE,
            0xFF00..=0xFF7F => AddrSpace::IO,
            0xFF80..=0xFFFE => AddrSpace::ZP,
            _ => return Err(BusError::NotImplemented),
        };

        Ok(region)
    }
}
