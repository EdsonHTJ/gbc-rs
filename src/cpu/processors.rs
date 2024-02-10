use crate::bus::BUS;
use crate::cpu::CPU;
use crate::cpu::error::CpuError;
use crate::instructions::{CondType, InType};
use crate::util;

impl CPU {

    fn get_z_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 7);
    }

    fn get_c_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 4);
    }

    fn check_condition(&self) -> bool {
        let condition = match self.current_instruction.cond {
            None => { return true; }
            Some(c) => {c}
        };
        
        let z = self.get_z_flag();
        let c = self.get_c_flag();

        return match condition {
            CondType::CtNz => { !z }
            CondType::CtZ => { z }
            CondType::CtNc => { !c }
            CondType::CtC => { c }
        }
    }

    fn process_ld(&self) -> Result<u32, CpuError> {
        todo!()
    }

    fn process_jp(&mut self) -> Result<u32, CpuError>{
        if !self.check_condition() {
            return Ok(0)
        }
        self.registers.pc = self.fetch_data;
        Ok(1)
    }

    pub(crate) fn process_instruction(&mut self, _bus: &mut BUS) -> Result<u32, CpuError> {
        return match self.current_instruction.type_ {
            InType::InNop {..} => {Ok(0)}
            InType::InLd => {
                return self.process_ld();
            }
            InType::InJp => {
                return self.process_jp();
            }
            _ => {Err(CpuError::InvalidInstruction)}
        }
    }
}