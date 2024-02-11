use crate::bus::BUS;
use crate::cpu::CPU;
use crate::cpu::error::CpuError;

impl CPU {
    pub fn stack_push(&mut self, bus: &mut BUS, data: u8) -> Result<(), CpuError> {
        self.registers.sp -= 1;
        bus.write(self.registers.sp, data)?;
        Ok(())
    }

    pub fn stack_pop(&mut self, bus: &mut BUS) -> Result<u8, CpuError> {
        let data = bus.read(self.registers.sp)?;
        self.registers.sp += 1;
        Ok(data)
    }

    pub fn stack_push_16(&mut self, bus: &mut BUS, data: u16) -> Result<(), CpuError> {
        self.stack_push(bus, (data >> 8) as u8)?;
        self.stack_push(bus, (data & 0xFF) as u8)?;
        Ok(())
    }

    pub fn stack_pop_16(&mut self, bus: &mut BUS) -> Result<u16, CpuError> {
        let low = self.stack_pop(bus)? as u16;
        let high = self.stack_pop(bus)? as u16;
        Ok((high << 8) | low)
    }
}