use crate::bus::{BUS, BusError};
use crate::bus::addresses::AddrSpace;

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
        Ok(0)
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



pub fn get_writer_by_region(region: AddrSpace) -> Result<WriterPtr, BusError> {
    match region {
        AddrSpace::ROM0 | AddrSpace::ROM1  | AddrSpace::CRAM=> {
            Ok(Box::new(CartridgeWriter{}))
        },
        AddrSpace::BG1 | AddrSpace::BG2 => {
            todo!()
        },
        AddrSpace::RAM0 | AddrSpace::RAM1 => {
            todo!()
        }
        AddrSpace::ECHO => {
            Ok(Box::new(NoneWriter{}))
        }
        AddrSpace::OAM => {
            todo!()
        }
        AddrSpace::UNUSABLE => {
            Ok(Box::new(NoneWriter{}))
        }
        AddrSpace::IO => {
            todo!()
        }
        AddrSpace::ZP => {
            todo!()
        }
        _ => Err(BusError::NotImplemented),
    }
}