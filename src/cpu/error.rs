use crate::bus::BusError;

#[derive(Debug)]
pub enum CpuError {
    BusError(BusError),
    UnknownAddressMode,
    InvalidInstruction(u32),
    InvalidRegister,
    InvalidCb(u8)
}

impl From<BusError> for CpuError {
    fn from(error: BusError) -> Self {
        CpuError::BusError(error)
    }
}
