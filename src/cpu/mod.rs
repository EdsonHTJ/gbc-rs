mod error;
mod fetch;
mod flags;
pub mod interrupts;
mod processors;
mod stack;


use std::sync::{Arc, Mutex};
use crate::bus::{BusError, BusMutex};
use crate::cartridge::ROM_HEADER_START;
use crate::cpu::error::CpuError;
use crate::cpu::interrupts::IFlagsRegister;
use crate::debug::{formatter, trace};
use crate::instructions::{Instruction, RegType};
use crate::tick::TickManager;

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
    pub enable_ime: bool,
    pub int_flags: Arc<Mutex<IFlagsRegister>>,
    pub ie_register: Arc<Mutex<IFlagsRegister>>,
    pub interrupt_master_enable: bool,
    pub stopped: bool,
    pub bus: BusMutex,
    pub tm: TickManager,
    pub previous_pc: u16,
}

impl CPU {
    pub fn new(bus: BusMutex, tm: TickManager, int_flags: Arc<Mutex<IFlagsRegister>>, ie_register: Arc<Mutex<IFlagsRegister>>) -> CPU {
        CPU {
            registers: CpuRegisters {
                a: 0x01,
                f: 0xB0,
                b: 0,
                c: 0x13,
                d: 0,
                e: 0xD8,
                h: 0x01,
                l: 0x4d,
                pc: ROM_HEADER_START as u16,
                sp: 0xFFFE,
            },
            fetch_data: 0,
            mem_dest: 0,
            current_opcode: 0,
            halted: false,
            stepping: false,
            dest_is_mem: false,
            current_instruction: Instruction::new(),
            enable_ime: false,
            stopped: false,
            int_flags,
            ie_register,
            interrupt_master_enable: false,
            previous_pc: ROM_HEADER_START as u16,
            tm,
            bus,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        //Create a new cpu and clone its fields to self
        let mut new_cpu = CPU::new(self.bus.clone(), self.tm.clone(), self.int_flags.clone(), self.ie_register.clone());
        std::mem::swap(self, &mut new_cpu);
    }

    pub fn cycle(&mut self, cycles: u32) {
        self.tm.cycle(cycles);
    }

    pub fn read_register(&self, reg_type: Option<RegType>) -> Result<u16, CpuError> {
        let reg_type = match reg_type {
            None => return Err(CpuError::InvalidRegister),
            Some(r) => r,
        };

        let value = match reg_type {
            RegType::RtA => self.registers.a as u16,
            RegType::RtF => self.registers.f as u16,
            RegType::RtB => self.registers.b as u16,
            RegType::RtC => self.registers.c as u16,
            RegType::RtD => self.registers.d as u16,
            RegType::RtE => self.registers.e as u16,
            RegType::RtH => self.registers.h as u16,
            RegType::RtL => self.registers.l as u16,
            RegType::RtSp => self.registers.sp,
            RegType::RtPc => self.registers.pc,
            RegType::RtAf => {
                let high = self.registers.a as u16;
                let low = self.registers.f as u16;
                let value = (high << 8) | low;
                value
            }
            RegType::RtBc => {
                let high = self.registers.b as u16;
                let low = self.registers.c as u16;
                let value = (high << 8) | low;
                value
            }
            RegType::RtDe => {
                let high = self.registers.d as u16;
                let low = self.registers.e as u16;
                let value = (high << 8) | low;
                value
            }
            RegType::RtHl => {
                let high = self.registers.h as u16;
                let low = self.registers.l as u16;
                let value = (high << 8) | low;
                value
            }
        };

        Ok(value)
    }

    pub fn write_register(
        &mut self,
        reg_type: Option<RegType>,
        value: u16,
    ) -> Result<(), CpuError> {
        let reg_type = match reg_type {
            None => return Err(CpuError::InvalidRegister),
            Some(r) => r,
        };

        match reg_type {
            RegType::RtA => self.registers.a = value as u8,
            RegType::RtF => self.registers.f = value as u8,
            RegType::RtB => self.registers.b = value as u8,
            RegType::RtC => self.registers.c = value as u8,
            RegType::RtD => self.registers.d = value as u8,
            RegType::RtE => self.registers.e = value as u8,
            RegType::RtH => self.registers.h = value as u8,
            RegType::RtL => self.registers.l = value as u8,
            RegType::RtAf => {
                self.registers.a = (value >> 8) as u8;
                self.registers.f = value as u8;
            }
            RegType::RtBc => {
                self.registers.b = (value >> 8) as u8;
                self.registers.c = value as u8;
            }
            RegType::RtDe => {
                self.registers.d = (value >> 8) as u8;
                self.registers.e = value as u8;
            }
            RegType::RtHl => {
                self.registers.h = (value >> 8) as u8;
                self.registers.l = value as u8;
            }
            RegType::RtSp => self.registers.sp = value,
            RegType::RtPc => self.registers.pc = value,
        };

        Ok(())
    }

    pub fn read_register_r1(&self) -> Result<u16, CpuError> {
        self.read_register(self.current_instruction.reg_1)
    }

    pub fn write_register_r1(&mut self, value: u16) -> Result<(), CpuError> {
        self.write_register(self.current_instruction.reg_1, value)
    }

    pub fn read_register_r2(&self) -> Result<u16, CpuError> {
        self.read_register(self.current_instruction.reg_2)
    }

    #[allow(dead_code)]
    pub fn write_register_r2(&mut self, value: u16) -> Result<(), CpuError> {
        self.write_register(self.current_instruction.reg_2, value)
    }

    pub fn cpu_read_r8(&mut self, reg: Option<RegType>) -> Result<u8, CpuError> {
        let reg = match reg {
            None => return Err(CpuError::InvalidRegister),
            Some(r) => r,
        };

        let val = match reg {
            RegType::RtA => self.registers.a,
            RegType::RtF => self.registers.f,
            RegType::RtB => self.registers.b,
            RegType::RtC => self.registers.c,
            RegType::RtD => self.registers.d,
            RegType::RtE => self.registers.e,
            RegType::RtH => self.registers.h,
            RegType::RtL => self.registers.l,
            RegType::RtHl => {
                let addr = self.read_register(Some(RegType::RtHl))?;
                return Ok(self.bus.read(addr)?);
            }
            _ => return Err(CpuError::InvalidRegister),
        };

        Ok(val)
    }

    pub fn cpu_write_r8(
        &mut self,
        reg: Option<RegType>,
        value: u8,
    ) -> Result<(), CpuError> {
        let reg = match reg {
            None => return Err(CpuError::InvalidRegister),
            Some(r) => r,
        };

        match reg {
            RegType::RtA => self.registers.a = value,
            RegType::RtF => self.registers.f = value,
            RegType::RtB => self.registers.b = value,
            RegType::RtC => self.registers.c = value,
            RegType::RtD => self.registers.d = value,
            RegType::RtE => self.registers.e = value,
            RegType::RtH => self.registers.h = value,
            RegType::RtL => self.registers.l = value,
            RegType::RtHl => {
                let addr = self.read_register(Some(RegType::RtHl))?;
                self.bus.write(addr, value)?;
            }
            _ => return Err(CpuError::InvalidRegister),
        };

        Ok(())
    }

    pub fn execute(&mut self) -> Result<u32, CpuError> {
        return self.process_instruction();
    }

    pub fn step_cpu(&mut self) -> Result<(), CpuError> {
        if !self.halted {
            self.fetch_instruction()?;
            self.cycle(1);
            self.fetch_data()?;
            //Logger::log_cpu_state_with_instruction(&self);
            let log = formatter::format_cpu_state(&self);
            trace::Trace::log_static(log.clone());
           // println!("{}", log);
            self.execute()?;
        } else {
            self.tm.cycle(1);
            if self.int_flags.lock().unwrap().int_flags != 0 {
                self.halted = false;
            }
        }

        if self.interrupt_master_enable
        {
            self.handler_interrupts()?;
            self.enable_ime = false;
        }

        if self.enable_ime {
            self.interrupt_master_enable = true;
        }

        Ok(())
    }

}
