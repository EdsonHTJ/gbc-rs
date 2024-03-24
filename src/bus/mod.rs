mod addresses;
mod writers;

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use crate::bus::addresses::AddrSpace;
use crate::cartridge::{Cartridge, CARTRIDGE_SINGLETON, CartridgeError};
use crate::cpu::interrupts::{IFlagsRegister, INTERRUPT_ENABLE};
use crate::dma::DMA;
use crate::emu::GlobalContext;
use crate::io::{IO, IoError};
use crate::ppu::{PPU, PPU_SINGLETON};
use crate::ram::{Ram, RamError};

#[derive(Debug)]
pub enum BusError {
    NoCartridgeLoaded,
    CartridgeError(CartridgeError),
    InvalidAddress,
    RamError(RamError),
    MutexError,
    IoError(IoError),
}

impl From<CartridgeError> for BusError {
    fn from(e: CartridgeError) -> BusError {
        BusError::CartridgeError(e)
    }
}

impl From<RamError> for BusError {
    fn from(e: RamError) -> BusError {
        BusError::RamError(e)
    }
}

impl From<PoisonError<MutexGuard<'_, BUS>>> for BusError {
    fn from(_: PoisonError<MutexGuard<'_, BUS>>) -> BusError {
        BusError::MutexError
    }
}

impl From<IoError> for BusError {
    fn from(e: IoError) -> BusError {
        BusError::IoError(e)
    }
}

#[derive(Clone)]
pub struct BusMutex {
    pub bus: Arc<Mutex<BUS>>,
}

impl BusMutex {
    pub fn new(global_context: GlobalContext) -> BusMutex {
        BusMutex {
            bus: Arc::new(Mutex::new(BUS::new(global_context))),
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, BusError> {
        let mut bus = self.bus.lock()?;
        bus.read(address)
    }

    #[allow(dead_code)]
    pub fn read_16(&self, address: u16) -> Result<u16, BusError> {
        let mut bus = self.bus.lock()?;
        bus.read_16(address)
    }

    pub fn write(&self, address: u16, data: u8) -> Result<(), BusError> {
        let mut bus = self.bus.lock()?;
        bus.write(address, data)
    }

    pub fn write_16(&self, address: u16, data: u16) -> Result<(), BusError> {
        let mut bus = self.bus.lock()?;
        bus.write_16(address, data)
    }
}

pub struct BUS {
    ram: Ram,
    dma: Arc<Mutex<DMA>>,
    io: Arc<Mutex<IO>>,
}

impl BUS {
    pub fn new(global_context: GlobalContext) -> BUS {
        BUS {
            ram: Ram::new(),
            io: global_context.io.unwrap(),
            dma: global_context.dma.unwrap(),
        }
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

    #[allow(dead_code)]
    pub fn read_16(&mut self, address: u16) -> Result<u16, BusError> {
        let low = self.read(address)? as u16;
        let high = self.read(address + 1)? as u16;
        Ok((high << 8) | low)
    }

    fn read_from_cartridge(&mut self, address: u16) -> Result<u8, BusError> {
        let binding = CARTRIDGE_SINGLETON.lock().unwrap();
        let cartridge = binding.as_ref().unwrap();

        cartridge
            .read(address)
            .map_err(|e| BusError::CartridgeError(e))
    }

    fn write_to_cartridge(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let mut binding = CARTRIDGE_SINGLETON.lock().unwrap();
        let mut cartridge = binding.as_mut().unwrap();

        cartridge
            .write(address, data)
            .map_err(|e| BusError::CartridgeError(e))
    }

    fn read_from_ram(&self, address: u16) -> Result<u8, BusError> {
        let region = AddrSpace::from_address(&address)?;
        match region {
            AddrSpace::RAM0 | AddrSpace::RAM1 =>  {
                let (start, _) = AddrSpace::RAM0.get_region();
                Ok(self.ram.read_wram(address - start)?)
            },
            AddrSpace::ZP => Ok(self.ram.read_hram(AddrSpace::get_region_offset(address)?)?),
            _ => Err(BusError::InvalidAddress),
        }
    }

    fn write_to_ram(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let region = AddrSpace::from_address(&address)?;
        match region {
            AddrSpace::RAM0 | AddrSpace::RAM1 =>  {
                let (start, _ ) = AddrSpace::RAM0.get_region();
                self.ram.write_wram(address - start, data)?
            },
            AddrSpace::ZP => self.ram.write_hram(AddrSpace::get_region_offset(address)?, data)?,
            _ => return Err(BusError::InvalidAddress),
        }

        Ok(())
    }

    fn read_from_master_interruption_register(&self) -> u8 {
        INTERRUPT_ENABLE.lock().unwrap().int_flags
    }

    fn write_to_master_interruption_register(&mut self, data: u8) {
        INTERRUPT_ENABLE.lock().unwrap().int_flags = data;
    }

    fn read_from_io(&self, address: u16) -> Result<u8, BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::IO => Ok(self.io.lock().unwrap().read((address & 0xFF) as u8)?),
            _ => Err(BusError::InvalidAddress),
        }
    }

    fn write_to_io(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::IO => self.io.lock().unwrap().write((address &0xFF) as u8, data)?,
            _ => return Err(BusError::InvalidAddress),
        }

        Ok(())
    }

    fn write_to_oam(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        if self.dma.lock().unwrap().dma_transferring() {
            return Ok(());
        }

        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::OAM => PPU_SINGLETON.lock().unwrap().oam_write(address, data),
            _ => return Err(BusError::InvalidAddress),
        }

        Ok(())
    }

    fn read_from_oam(&self, address: u16) -> Result<u8, BusError> {
        if self.dma.lock().unwrap().dma_transferring() {
            return Ok(0xFF);
        }

        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::OAM => Ok(PPU_SINGLETON.lock().unwrap().oam_read(address)),
            _ => return Err(BusError::InvalidAddress),
        }
    }

    fn write_to_vram(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::VRAM => PPU_SINGLETON.lock().unwrap().vram_write(address, data),
            _ => return Err(BusError::InvalidAddress),
        }

        Ok(())
    }

    fn read_from_vram(&self, address: u16) -> Result<u8, BusError> {
        let region = AddrSpace::from_address(&address)?;
        let address = AddrSpace::get_region_offset(address)?;
        match region {
            AddrSpace::VRAM => Ok(PPU_SINGLETON.lock().unwrap().vram_read(address)),
            _ => return Err(BusError::InvalidAddress),
        }
    }
}
