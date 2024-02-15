use crate::bus::addresses::AddrSpace;
use crate::bus::{BusError, BUS};

pub trait BusWriter {
    fn write(&self, bus: &mut BUS, address: u16, data: u8) -> Result<(), BusError>;
    fn read(&self, bus: &mut BUS, address: u16) -> Result<u8, BusError>;
}

type WriterPtr = Box<dyn BusWriter>;

pub struct NoneWriter {}

impl BusWriter for NoneWriter {
    fn write(&self, _bus: &mut BUS, _address: u16, _data: u8) -> Result<(), BusError> {
        Ok(())
    }

    fn read(&self, _bus: &mut BUS, _address: u16) -> Result<u8, BusError> {
        //panic!("Trying to read from a non readable region")
        Ok(0)
    }
}

pub struct InterruptionWriter {}

impl BusWriter for InterruptionWriter {
    fn write(&self, bus: &mut BUS, _address: u16, data: u8) -> Result<(), BusError> {
        Ok(bus.write_to_master_interruption_register(data))
    }

    fn read(&self, bus: &mut BUS, _address: u16) -> Result<u8, BusError> {
        Ok(bus.read_from_master_interruption_register())
    }
}

pub struct CartridgeWriter {}

impl BusWriter for CartridgeWriter {
    fn write(&self, bus: &mut BUS, address: u16, data: u8) -> Result<(), BusError> {
        bus.write_to_cartridge(address, data)
    }

    fn read(&self, bus: &mut BUS, address: u16) -> Result<u8, BusError> {
        bus.read_from_cartridge(address)
    }
}

pub struct WRamWriter {}

impl BusWriter for WRamWriter {
    fn write(&self, bus: &mut BUS, address: u16, data: u8) -> Result<(), BusError> {
        bus.write_to_ram(address, data)
    }

    fn read(&self, bus: &mut BUS, address: u16) -> Result<u8, BusError> {
        bus.read_from_ram(address)
    }
}

pub struct IoWriter{}

impl BusWriter for IoWriter {
    fn write(&self, bus: &mut BUS, address: u16, data: u8) -> Result<(), BusError> {
        bus.write_to_io(address, data)
    }

    fn read(&self, bus: &mut BUS, address: u16) -> Result<u8, BusError> {
        bus.read_from_io(address)
    }
}


pub fn get_writer_by_region(region: AddrSpace) -> Result<WriterPtr, BusError> {
    match region {
        AddrSpace::ROM0 | AddrSpace::ROM1 | AddrSpace::CRAM => Ok(Box::new(CartridgeWriter {})),
        AddrSpace::BG1 | AddrSpace::BG2 => Ok(Box::new(NoneWriter {})),
        AddrSpace::RAM0 | AddrSpace::RAM1 | AddrSpace::ZP => Ok(Box::new(WRamWriter {})),
        AddrSpace::ECHO => Ok(Box::new(NoneWriter {})),
        AddrSpace::OAM => Ok(Box::new(NoneWriter {})),
        AddrSpace::UNUSABLE => Ok(Box::new(NoneWriter {})),
        AddrSpace::IO => Ok(Box::new(IoWriter {})),
        AddrSpace::INTERRUPT => Ok(Box::new(InterruptionWriter {})),
        _ => Ok(Box::new(NoneWriter {})),
    }
}
