use crate::bus::BusError;
use crate::tick::TickError;

#[derive(Debug)]
pub enum CpuError {
    BusError(BusError),
    InvalidInstruction(u32),
    InvalidRegister,
    InvalidCb(u8),
    TickError(TickError),
}

impl From<BusError> for CpuError {
    fn from(error: BusError) -> Self {
        CpuError::BusError(error)
    }
}

impl From<TickError> for CpuError {
    fn from(error: TickError) -> Self {
        CpuError::TickError(error)
    }
}
