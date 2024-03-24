use crate::bus::BUS_SINGLETON;
use crate::cpu::error::CpuError;
use crate::cpu::CPU;
use crate::instructions::{AddrMode, Instruction, RegType};

impl CPU {
    pub fn fetch_instruction(&mut self) -> Result<(), CpuError> {
        self.current_opcode = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)?;
        self.current_instruction = Instruction::by_opcode(self.current_opcode)
            .ok_or(CpuError::InvalidInstruction(self.current_opcode as u32))?;
        self.previous_pc = self.registers.pc;
        self.registers.pc += 1;
        Ok(())
    }

    pub fn fetch_data(&mut self) -> Result<u32, CpuError> {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        return match self.current_instruction.mode {
            AddrMode::AmImp => Ok(0),
            AddrMode::AmR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_1)?;
                Ok(0)
            }
            AddrMode::AmRR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                Ok(0)
            }
            AddrMode::AmRD8 => {
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;

                self.cycle(1);
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmD16 | AddrMode::AmRD16 => {
                let low = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;

                self.cycle(1);
                self.registers.pc += 1;

                let high = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;

                self.fetch_data = (high << 8) | low;
                Ok(2)
            }
            AddrMode::AmMRR => {
                self.fetch_data = self.read_register(self.current_instruction.reg_2)?;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;

                if self.current_instruction.reg_1 == Some(RegType::RtC) {
                    self.mem_dest |= 0xFF00;
                }

                Ok(0)
            }
            AddrMode::AmRMR => {
                let addr = self.read_register(self.current_instruction.reg_2)?;
                if self.current_instruction.reg_1 == Some(RegType::RtC) {
                    self.mem_dest |= 0xFF00;
                }

                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(addr)? as u16;
                self.cycle(1);
                Ok(1)
            }
            AddrMode::AmRHli => {
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.read_register(Some(RegType::RtHl))?)? as u16;
                self.cycle(1);
                self.write_register(
                    Some(RegType::RtHl),
                    self.read_register(Some(RegType::RtHl))? + 1,
                )?;
                Ok(1)
            }
            AddrMode::AmRHld => {
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.read_register(Some(RegType::RtHl))?)? as u16;
                self.cycle(1);
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
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmA8R => {
                self.mem_dest = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.dest_is_mem = true;
                self.cycle(1);
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmHlSpr => {
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmD8 => {
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;
                Ok(1)
            }
            AddrMode::AmA16R | AddrMode::AmD16R => {
                let low = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;

                let high = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
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
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                Ok(1)
            }
            AddrMode::AmMr => {
                self.mem_dest = self.read_register(self.current_instruction.reg_1)?;
                self.dest_is_mem = true;
                self.fetch_data =
                    BUS_SINGLETON.lock().unwrap().read(self.read_register(self.current_instruction.reg_1)?)? as u16;
                self.cycle(1);
                Ok(1)
            }
            AddrMode::AmRA16 => {
                let low = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;

                let high = BUS_SINGLETON.lock().unwrap().read(self.registers.pc)? as u16;
                self.cycle(1);
                self.registers.pc += 1;

                let addr = (high << 8) | low;
                self.fetch_data = BUS_SINGLETON.lock().unwrap().read(addr)? as u16;
                self.cycle(1);
                Ok(3)
            }
        };
    }
}
