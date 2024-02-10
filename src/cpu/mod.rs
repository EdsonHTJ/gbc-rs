mod error;
mod processors;

use crate::cpu::error::CpuError;
use crate::bus::{BUS};
use crate::cartridge::ROM_HEADER_START;
use crate::instructions::{AddrMode, Instruction, RegType};


pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub struct CPU {
    pub registers: CpuRegisters,
    pub fetch_data: u16,
    pub mem_dest: u16,
    pub current_opcode: u8,
    pub halted: bool,
    pub stepping: bool,
    pub dest_is_mem: bool,
    pub current_instruction: Instruction,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: CpuRegisters {
                a: 0,
                f: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                pc: ROM_HEADER_START as u16,
                sp: 0,
            },
            fetch_data: 0,
            mem_dest: 0,
            current_opcode: 0,
            halted: false,
            stepping: false,
            dest_is_mem: false,
            current_instruction: Instruction::new(),
        }
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.f = 0;
        self.registers.b = 0;
        self.registers.c = 0;
        self.registers.d = 0;
        self.registers.e = 0;
        self.registers.h = 0;
        self.registers.l = 0;
        self.registers.pc = ROM_HEADER_START as u16;
        self.registers.sp = 0;
        self.fetch_data = 0;
        self.mem_dest = 0;
        self.current_opcode = 0;
        self.halted = false;
        self.stepping = false;
        self.dest_is_mem = false;
        self.current_instruction = Instruction::new();
    }

    pub fn read_register(&self, reg_type: Option<RegType>) -> Result<u16, CpuError> {
        let reg_type = match reg_type {
            None => { return Err(CpuError::InvalidRegister) }
            Some(r) => {r}
        };

        let value = match reg_type {
            RegType::RtA => { self.registers.a as u16 }
            RegType::RtF => { self.registers.f as u16 }
            RegType::RtB => { self.registers.b as u16 }
            RegType::RtC => { self.registers.c as u16 }
            RegType::RtD => { self.registers.d as u16 }
            RegType::RtE => { self.registers.e as u16 }
            RegType::RtH => { self.registers.h as u16 }
            RegType::RtL => { self.registers.l as u16 }
            RegType::RtAf => { (self.registers.a as u16) << 8 | self.registers.f as u16 }
            RegType::RtBc => { (self.registers.b as u16) << 8 | self.registers.c as u16 }
            RegType::RtDe => { (self.registers.d as u16) << 8 | self.registers.e as u16 }
            RegType::RtHl => { (self.registers.h as u16) << 8 | self.registers.l as u16 }
            RegType::RtSp => { self.registers.sp }
            RegType::RtPc => { self.registers.pc }
        };

        Ok(value)
    }

    pub fn fetch_instruction(&mut self, bus: &mut BUS) -> Result<(), CpuError> {
        self.current_opcode = bus.read(self.registers.pc)?;
        self.registers.pc += 1;
        self.current_instruction = Instruction::by_opcode(self.current_opcode).ok_or(CpuError::InvalidInstruction)?;
        Ok(())
    }

    pub fn fetch_data(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        return match self.current_instruction.mode {
            AddrMode::AmImp => { Ok((0)) }
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
            AddrMode::AmD16 => {
                self.fetch_data = bus.read(self.registers.pc)? as u16;
                //Update emulation cycles by 1

                self.fetch_data |= (bus.read(self.registers.pc + 1)? as u16) << 8;
                //Update emulation cycles by 1

                self.registers.pc += 2;

                Ok(2)
            }
            _ => {
                Err(CpuError::UnknownAddressMode)
            }
        }
    }

    pub fn execute(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        return self.process_instruction(bus);
    }

}