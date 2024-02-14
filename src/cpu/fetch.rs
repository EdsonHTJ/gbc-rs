use crate::bus::BUS;
use crate::cpu::error::CpuError;
use crate::cpu::CPU;
use crate::instructions::{AddrMode, Instruction, RegType};
use crate::log::{Logger, LoggerTrait};

impl CPU {
    pub fn fetch_instruction(&mut self, bus: &mut BUS) -> Result<(), CpuError> {
        self.current_opcode = bus.read(self.registers.pc)?;
        self.current_instruction = Instruction::by_opcode(self.current_opcode)
            .ok_or(CpuError::InvalidInstruction(self.current_opcode as u32))?;
        self.registers.pc += 1;
        Ok(())
    }

    pub fn fetch_data(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        if self.current_opcode == 0xFA {
            println!("Fetching data for opcode 0xFA");
        }

        return match self.current_instruction.mode {
            AddrMode::AmImp => Ok((0)),
            AddrMode::AmR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_1)?;
                Ok(0)
            }
            AddrMode::AmRR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                Ok(0)
            }
            AddrMode::AmRD8 => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;

                //Update emulation cycles by 1
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmD16 | AddrMode::AmRD16 => {
                let low = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                let high = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                self.fetch_data = (high << 8) | low;
                Ok(2)
            }
            AddrMode::AmMRR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;

                if (self.current_instruction.reg_1 == Some(RegType::RtC)) {
                    self.mem_dest |= 0xFF00;
                }

                Ok(0)
            }
            AddrMode::AmRMR => {
                let addr = self.read_register(self.current_instruction.reg_2)?;
                if (self.current_instruction.reg_1 == Some(RegType::RtC)) {
                    self.mem_dest |= 0xFF00;
                }

                self.fetch_data = bus.read(addr)? as u16;
                Ok(1)
            }
            AddrMode::AmRHli => {
                self.fetch_data = bus.read(self.read_register(Some(RegType::RtHl))?)? as u16;
                self.write_register(
                    Some(RegType::RtHl),
                    self.read_register(Some(RegType::RtHl))? + 1,
                )?;
                Ok(1)
            }
            AddrMode::AmRHld => {
                self.fetch_data = bus.read(self.read_register(Some(RegType::RtHl))?)? as u16;
                self.write_register(
                    Some(RegType::RtHl),
                    self.read_register(Some(RegType::RtHl))? - 1,
                )?;
                Ok(1)
            }
            AddrMode::AmHliR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                self.write_register(
                    Some(RegType::RtHl),
                    self.read_register(Some(RegType::RtHl))? + 1,
                )?;
                Ok(0)
            }
            AddrMode::AmHldR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                self.write_register(
                    Some(RegType::RtHl),
                    self.read_register(Some(RegType::RtHl))? - 1,
                )?;
                Ok(0)
            }
            AddrMode::AmRA8 => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmA8R => {
                self.mem_dest = bus.read(self.registers.pc)? as u16;
                self.dest_is_mem = true;
                //Update emulation cycles by 1
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmHlSpr => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmD8 => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmA16R | AddrMode::AmD16R => {
                let low = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                let high = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                self.mem_dest = (high << 8) | low;
                self.dest_is_mem = true;
                if self.current_instruction.reg_2.is_some() {
                    self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                } else {
                    self.fetch_data = 0;
                }
                Ok(2)
            }
            AddrMode::AmMrD8 => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                Ok(1)
            }
            AddrMode::AmMr => {
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                self.fetch_data =
                    bus.read(self.read_register(self.current_instruction.reg_1)?)? as u16;
                //Update emulation cycles by 1
                Ok(1)
            }
            AddrMode::AmRA16 => {
                let low = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                let high = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1
                self.registers.pc += 1;

                let addr = (high << 8) | low;
                self.fetch_data = bus.read(addr)? as u16;
                //Update emulation cycles by 1
                Ok(3)
            }
        };
    }
}
