use crate::bus::BUS_SINGLETON;
use crate::cpu::error::CpuError;
use crate::cpu::CPU;

impl CPU {
    pub fn stack_push(&mut self, data: u8) -> Result<(), CpuError> {
        self.registers.sp -= 1;
        BUS_SINGLETON.lock().unwrap().write(self.registers.sp, data)?;
        Ok(())
    }

    pub fn stack_pop(&mut self) -> Result<u8, CpuError> {
        let data = BUS_SINGLETON.lock().unwrap().read(self.registers.sp)?;
        self.registers.sp += 1;
        Ok(data)
    }

    pub fn stack_push_16(&mut self, data: u16) -> Result<(), CpuError> {
        self.stack_push((data >> 8) as u8)?;
        self.stack_push((data & 0xFF) as u8)?;
        Ok(())
    }

    pub fn stack_pop_16(&mut self) -> Result<u16, CpuError> {
        let low = self.stack_pop()? as u16;
        let high = self.stack_pop()? as u16;
        Ok((high << 8) | low)
    }
}
