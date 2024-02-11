use crate::bus::BUS;
use crate::cpu::error::CpuError;
use crate::cpu::flags::FlagMode;
use crate::cpu::CPU;
use crate::instructions::{CondType, InType};
use crate::util;

impl CPU {
    fn check_condition(&self) -> bool {
        let condition = match self.current_instruction.cond {
            None => {
                return true;
            }
            Some(c) => c,
        };

        let z = self.get_z_flag();
        let c = self.get_c_flag();

        return match condition {
            CondType::CtNz => !z,
            CondType::CtZ => z,
            CondType::CtNc => !c,
            CondType::CtC => c,
        };
    }

    fn process_ld(&self) -> Result<u32, CpuError> {
        todo!()
    }

    fn process_jp(&mut self) -> Result<u32, CpuError> {
        if !self.check_condition() {
            return Ok(0);
        }
        self.registers.pc = self.fetch_data;
        Ok(1)
    }

    fn process_di(&mut self) -> Result<u32, CpuError> {
        self.interrupt_master_enable = false;
        Ok(0)
    }

    fn process_ei(&mut self) -> Result<u32, CpuError> {
        self.interrupt_master_enable = true;
        Ok(0)
    }

    fn process_xor(&mut self) -> Result<u32, CpuError> {
        self.registers.a ^= (self.fetch_data & 0xFF) as u8;

        let z = FlagMode::from(self.registers.a == 0);
        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Clear, FlagMode::Clear);
        Ok(0)
    }

    pub(crate) fn process_instruction(&mut self, _bus: &mut BUS) -> Result<u32, CpuError> {
        return match self.current_instruction.type_ {
            InType::InNop { .. } => Ok(0),
            InType::InLd => {
                return self.process_ld();
            }
            InType::InJp => {
                return self.process_jp();
            }
            InType::InDi => {
                return self.process_di();
            }
            InType::InEi => {
                return self.process_ei();
            }
            InType::InXor => {
                return self.process_xor();
            }
            _ => Err(CpuError::InvalidInstruction(self.current_opcode as u32)),
        };
    }
}
