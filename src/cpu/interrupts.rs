use crate::bus::BUS;
use crate::cpu::error::CpuError;
use crate::cpu::CPU;

pub enum InterruptType {
    VBlank,
    LcdStart,
    Timer,
    Serial,
    JoyPad,
}

impl InterruptType {
    pub fn value_has_interrupt(&self, value: u32) -> bool {
        match self {
            InterruptType::VBlank => (value & 0x01) != 0,
            InterruptType::LcdStart => (value & 0x02) != 0,
            InterruptType::Timer => (value & 0x04) != 0,
            InterruptType::Serial => (value & 0x08) != 0,
            InterruptType::JoyPad => (value & 0x16) != 0,
        }
    }

    pub fn value_add_interrupt(&self, value: u32) -> u32 {
        match self {
            InterruptType::VBlank => value | 0x01,
            InterruptType::LcdStart => value | 0x02,
            InterruptType::Timer => value | 0x04,
            InterruptType::Serial => value | 0x08,
            InterruptType::JoyPad => value | 0x16,
        };

        value
    }

    pub fn value_remove_interrupt(&self, value: u32) -> u32 {
        match self {
            InterruptType::VBlank => value & 0xFE,
            InterruptType::LcdStart => value & 0xFD,
            InterruptType::Timer => value & 0xFB,
            InterruptType::Serial => value & 0xF7,
            InterruptType::JoyPad => value & 0xEF,
        };

        value
    }
}

impl CPU {
    pub fn interrupt_handler(&mut self, addr: u16, bus: &mut BUS) -> Result<(), CpuError> {
        self.stack_push_16(bus, self.registers.pc)?;
        self.registers.pc = addr;

        Ok(())
    }

    pub fn interruption_check(
        &mut self,
        addr: u16,
        interrupt_type: InterruptType,
        bus: &mut BUS,
    ) -> Result<bool, CpuError> {
        if (interrupt_type.value_has_interrupt(self.int_flags as u32)
            && interrupt_type.value_has_interrupt(self.ie_register as u32))
        {
            self.interrupt_handler(addr, bus)?;
            self.int_flags = interrupt_type.value_remove_interrupt(self.int_flags as u32) as u8;
            self.halted = false;
            self.set_interruption_master_enable(bus, 0)?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn handler_interrupts(&mut self, bus: &mut BUS) -> Result<(), CpuError> {
        if self.interruption_check(0x40, InterruptType::VBlank, bus)? {
            return Ok(());
        }
        if self.interruption_check(0x48, InterruptType::LcdStart, bus)? {
            return Ok(());
        }
        if self.interruption_check(0x50, InterruptType::Timer, bus)? {
            return Ok(());
        }
        if self.interruption_check(0x58, InterruptType::Serial, bus)? {
            return Ok(());
        }
        if self.interruption_check(0x60, InterruptType::JoyPad, bus)? {
            return Ok(());
        }
        Ok(())
    }
}