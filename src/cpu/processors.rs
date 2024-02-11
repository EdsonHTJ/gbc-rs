use crate::bus::BUS;
use crate::cpu::error::CpuError;
use crate::cpu::flags::FlagMode;
use crate::cpu::CPU;
use crate::instructions::{AddrMode, CondType, InType, RegType};
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

    fn process_ld(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        if self.dest_is_mem {
            if self.current_instruction.reg_2 > Some(RegType::RtAf) {
                cycles += 1;
                bus.write_16(self.mem_dest, self.fetch_data)?;
            } else {
                bus.write(self.mem_dest, self.fetch_data as u8)?;
            }
        }

        if self.current_instruction.mode == AddrMode::AmHlSpr {
            let hflag = (self.read_register_r2()? & 0xF) + (self.fetch_data & 0xF) >= 0x10;
            let cflag = (self.read_register_r2()? & 0xFF) + (self.fetch_data & 0xFF) >= 0x100;

            self.cpu_set_flags(
                FlagMode::Clear,
                FlagMode::Clear,
                FlagMode::from(hflag),
                FlagMode::from(cflag),
            );
            self.write_register_r1(
                ((self.read_register_r2()? as i16)
                    + i16::from_be_bytes(self.fetch_data.to_be_bytes())) as u16,
            )?;
        }

        Ok(cycles)
    }

    fn process_jp(&mut self) -> Result<u32, CpuError> {
        if !self.check_condition() {
            return Ok(0);
        }
        self.registers.pc = self.fetch_data;
        Ok(1)
    }

    fn process_di(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        bus.write(0xFFFF, 0x00)?; // Disable all interrupts
        Ok(0)
    }

    fn process_ei(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        bus.write(0xFFFF, 0x01)?; // Disable all interrupts
        Ok(0)
    }

    fn process_xor(&mut self) -> Result<u32, CpuError> {
        self.registers.a ^= (self.fetch_data & 0xFF) as u8;

        let z = FlagMode::from(self.registers.a == 0);
        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Clear, FlagMode::Clear);
        Ok(0)
    }

    fn process_ldh(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        if self.current_instruction.reg_1 == Some(RegType::RtA) {
            self.write_register(self.current_instruction.reg_1, bus.read(0xFF00 | self.fetch_data)? as u16)?;
        }else {
            bus.write(0xFF00 | self.fetch_data, self.registers.a)?;
        }

        Ok(1)
    }

    pub(crate) fn process_instruction(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        return match self.current_instruction.type_ {
            InType::InNop { .. } => Ok(0),
            InType::InLd => {
                return self.process_ld(bus);
            }
            InType::InJp => {
                return self.process_jp();
            }
            InType::InDi => {
                return self.process_di(bus);
            }
            InType::InEi => {
                return self.process_ei(bus);
            }
            InType::InXor => {
                return self.process_xor();
            }
            InType::InLdh => {
                return self.process_ldh(bus);
            }
            _ => Err(CpuError::InvalidInstruction(self.current_opcode as u32)),
        };
    }
}
