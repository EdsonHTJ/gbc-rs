use crate::bus::BusError;

#[derive(Debug)]
pub enum CpuError {
    BusError(BusError),
    UnknownAddressMode,
    InvalidInstruction,
    InvalidRegister
}

impl From<BusError> for CpuError {
    fn from(error: BusError) -> Self {
        CpuError::BusError(error)
    }
}