use crate::cpu::error::CpuError;
use crate::cpu::CPU;

pub enum InterruptType {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    JoyPad,
}

impl InterruptType {
    pub fn value_has_interrupt(&self, value: u32) -> bool {
        match self {
            InterruptType::VBlank => (value & 0x01) != 0,
            InterruptType::LcdStat => (value & 0x02) != 0,
            InterruptType::Timer => (value & 0x04) != 0,
            InterruptType::Serial => (value & 0x08) != 0,
            InterruptType::JoyPad => (value & 0x16) != 0,
        }
    }

    #[allow(dead_code)]
    pub fn value_add_interrupt(&self, value: u32) -> u32 {
        match self {
            InterruptType::VBlank => value | 0x01,
            InterruptType::LcdStat => value | 0x02,
            InterruptType::Timer => value | 0x04,
            InterruptType::Serial => value | 0x08,
            InterruptType::JoyPad => value | 0x16,
        }
    }

    pub fn value_remove_interrupt(&self, value: u32) -> u32 {
        match self {
            InterruptType::VBlank => value & 0xFE,
            InterruptType::LcdStat => value & 0xFD,
            InterruptType::Timer => value & 0xFB,
            InterruptType::Serial => value & 0xF7,
            InterruptType::JoyPad => value & 0xEF,
        }
    }
}

#[derive(PartialEq)]
pub struct IFlagsRegister {
    pub int_flags: u8,
}

impl IFlagsRegister {
    pub fn new() -> IFlagsRegister {
        IFlagsRegister { int_flags: 0 }
    }

    #[allow(dead_code)]
    pub fn has_interrupt(&self, interrupt_type: InterruptType) -> bool {
        interrupt_type.value_has_interrupt(self.int_flags as u32)
    }

    pub fn add_interrupt(&mut self, interrupt_type: InterruptType) {
        self.int_flags = interrupt_type.value_add_interrupt(self.int_flags as u32) as u8;
    }

    pub fn remove_interrupt(&mut self, interrupt_type: InterruptType) {
        self.int_flags = interrupt_type.value_remove_interrupt(self.int_flags as u32) as u8;
    }
}

impl CPU {
    pub fn interrupt_handler(&mut self, addr: u16) -> Result<(), CpuError> {
        self.stack_push_16(self.registers.pc)?;
        self.registers.pc = addr;

        Ok(())
    }

    pub fn interruption_check(
        &mut self,
        addr: u16,
        interrupt_type: InterruptType,
    ) -> Result<bool, CpuError> {
        let int_flags = self.int_flags.lock().unwrap().int_flags;
        let ie_flags = self.ie_register.lock().unwrap().int_flags;

        if interrupt_type.value_has_interrupt(int_flags as u32) && interrupt_type.value_has_interrupt(ie_flags as u32) {
            self.interrupt_handler(addr)?;
            self.int_flags.lock().unwrap().remove_interrupt(interrupt_type);
            self.interrupt_master_enable = false;
            self.halted = false;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn handler_interrupts(&mut self) -> Result<(), CpuError> {
        if self.interruption_check(0x40, InterruptType::VBlank)? {
            return Ok(());
        }
        if self.interruption_check(0x48, InterruptType::LcdStat)? {
            return Ok(());
        }
        if self.interruption_check(0x50, InterruptType::Timer)? {
            return Ok(());
        }
        if self.interruption_check(0x58, InterruptType::Serial)? {
            return Ok(());
        }
        if self.interruption_check(0x60, InterruptType::JoyPad)? {
            return Ok(());
        }
        Ok(())
    }
}
