use crate::bus::BUS;
use crate::cpu::error::CpuError;
use crate::cpu::flags::FlagMode;
use crate::cpu::CPU;
use crate::instructions::{AddrMode, CondType, InType, RegType};
use crate::instructions::cb::{CbOperation, CbType};
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
            if RegType::is_some_16_bit(self.current_instruction.reg_1){
                cycles += 1;
                bus.write_16(self.mem_dest, self.fetch_data)?;
            } else {
                bus.write(self.mem_dest, self.fetch_data as u8)?;
            }

            return Ok(cycles);
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

            let val_to_write = self.read_register_r2()?.wrapping_add_signed(i16::from_be_bytes(self.fetch_data.to_be_bytes()));
            self.write_register_r1(
                val_to_write
            )?;

            return Ok(cycles);
        }

        self.write_register(self.current_instruction.reg_1, self.fetch_data)?;
        Ok(cycles)
    }

    fn process_jp(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        self.goto_addr(self.fetch_data, false, bus)
    }

    fn process_di(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        bus.write(0xFFFF, 0x00)?; // Disable all interrupts
        Ok(0)
    }

    fn process_ei(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        bus.write(0xFFFF, 0x01)?; // Disable all interrupts
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

    fn process_pop(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let data = self.stack_pop_16(bus)?;
        self.write_register(self.current_instruction.reg_1, data)?;

        if self.current_instruction.reg_1 == Some(RegType::RtAf) {
            self.write_register(self.current_instruction.reg_1, data & 0xFFF0)?;
        }
        Ok(2)
    }

    fn process_push(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let data = self.read_register_r1()?;
        self.stack_push_16(bus, data)?;
        Ok(2)
    }

    fn goto_addr(&mut self, addr: u16, push_pc: bool, bus: &mut BUS) -> Result<u32, CpuError> {
        if !self.check_condition() {
            return Ok(0);
        }

        let mut cycles = 0;
        if push_pc {
            self.stack_push_16(bus, self.registers.pc)?;
            cycles += 2;
        }

        self.registers.pc = addr;
        Ok(cycles + 1)
    }

    fn process_call(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        self.goto_addr(self.fetch_data, true, bus)
    }

    fn process_jr(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let offset = i16::from_be_bytes(self.fetch_data.to_be_bytes());
        let next_pc = self.registers.pc.wrapping_add_signed(offset);
        self.goto_addr(next_pc, false, bus)
    }

    fn process_ret(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        if self.current_instruction.cond != None {
            cycles += 1;
        }

        if !self.check_condition() {
            return Ok(cycles);
        }

        let addr = self.stack_pop_16(bus)?;
        cycles += 2;
        self.registers.pc = addr;
        Ok(cycles)
    }

    fn process_reti(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        bus.write(0xFFFF, 0x01)?; // Enable all interrupts
        self.process_ret(bus)
    }

    fn process_rst(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        self.goto_addr(self.current_instruction.param as u16, true, bus)
    }

    fn process_inc(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut value = self.read_register_r1()? + 1;
        if RegType::is_some_16_bit(self.current_instruction.reg_1) {
            cycles += 1;
        }

        if (self.current_instruction.reg_1 == Some(RegType::RtHl)) && (self.current_instruction.mode == AddrMode::AmMr) {
            value = (bus.read(self.read_register_r1()?)? + 1) as u16;
            value &= 0xFF;
            bus.write(self.read_register_r1()?, value as u8)?;
        }else {
            self.write_register_r1(value)?;
            value = self.read_register_r1()?;
        }

        if self.current_opcode & 0x03 == 0x03 {
            return Ok(cycles);
        }

        self.cpu_set_flags(
            FlagMode::from(value == 0),
            FlagMode::Clear,
            FlagMode::from((value & 0xF) == 0),
            FlagMode::None,
        );
        Ok(cycles)
    }

    fn process_dec(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut value = self.read_register_r1()? - 1;
        if RegType::is_some_16_bit(self.current_instruction.reg_1) {
            cycles += 1;
        }

        if (self.current_instruction.reg_1 == Some(RegType::RtHl)) && (self.current_instruction.mode == AddrMode::AmMr) {
            value = (bus.read(self.read_register_r1()?)? - 1) as u16;
            value &= 0xFF;
            bus.write(self.read_register_r1()?, value as u8)?;
        }else {
            self.write_register_r1(value)?;
            value = self.read_register_r1()?;
        }

        if self.current_opcode & 0x0B == 0x0B {
            return Ok(cycles);
        }

        self.cpu_set_flags(
            FlagMode::from(value == 0),
            FlagMode::Set,
            FlagMode::from((value & 0xF) == 0x0F),
            FlagMode::None,
        );
        Ok(cycles)
    }

    fn process_add(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut val = self.read_register_r1()?.wrapping_add(self.fetch_data);
        let is_16_bit = RegType::is_some_16_bit(self.current_instruction.reg_1);

        if is_16_bit {
            cycles += 1;
        }

        if self.current_instruction.reg_1 == Some(RegType::RtSp) {
            val = (self.read_register_r1()?.wrapping_add_signed(i16::from_be_bytes(self.fetch_data.to_be_bytes())));
        }

        let z = val & 0xff == 0;
        let h = (self.read_register_r1()? & 0xf).wrapping_add(self.fetch_data & 0xf) > 0xf;
        let c = ((self.read_register_r1()? & 0xFF).wrapping_add_signed(i16::from_be_bytes((self.fetch_data & 0xFF).to_be_bytes())) > 0xFF);

        let mut z = FlagMode::from(z);
        let mut h = FlagMode::from(h);
        let mut c = FlagMode::from(c);

        if is_16_bit {
            z = FlagMode::None;
            let val_h = (self.read_register_r1()? & 0xFFF + self.fetch_data & 0xFFF >= 0x1000);
            h = FlagMode::from(val_h);

            let n = self.read_register_r1()? as u32 + self.fetch_data as u32;
            c = FlagMode::from(n >= 0x10000);
        }

        if self.current_instruction.reg_1 == Some(RegType::RtSp) {
            z = FlagMode::Clear;
            let val_h = (self.read_register_r1()? &0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10;
            h = FlagMode::from(val_h);
            let val_c = (self.read_register_r1()? & 0xFF).wrapping_add_signed(i16::from_be_bytes((self.fetch_data & 0xFF).to_be_bytes())) >= 0x100;
            c = FlagMode::from(val_c);
        }

        self.write_register(self.current_instruction.reg_1, val & 0xFFFF)?;
        self.cpu_set_flags(z, FlagMode::Clear, h, c);
        Ok(cycles)
    }

    fn process_adc(&mut self) -> Result<u32, CpuError> {
        let u = self.fetch_data;
        let a = self.registers.a as u16;
        let c = self.get_c_flag() as u16;

        let result = a + u + c;
        let z = FlagMode::from(result == 0);
        let h = FlagMode::from((a & 0xF) + (u & 0xF) + c > 0xF);
        let c = FlagMode::from(result > 0xFF);

        self.registers.a = result as u8;
        self.cpu_set_flags(z, FlagMode::Clear, h, c);
        Ok(0)
    }

    fn process_sub(&mut self) -> Result<u32, CpuError> {
        let val = self.read_register_r1()?.wrapping_add_signed(-i16::from_be_bytes(self.fetch_data.to_be_bytes()));

        let z = FlagMode::from(val == 0);
        let h = FlagMode::from((self.read_register_r1()? & 0xF) < (self.fetch_data & 0xF));
        let c = FlagMode::from(val > self.read_register_r1()?);

        self.write_register_r1(val)?;
        self.cpu_set_flags(z, FlagMode::Set, h, c);
        Ok(0)
    }

    fn process_sbc(&mut self) -> Result<u32, CpuError> {
        let u = self.fetch_data;
        let a = self.registers.a as u16;
        let c = self.get_c_flag() as u16;

        let result = a - u - c;
        let z = FlagMode::from(result == 0);
        let h = FlagMode::from((a & 0xF) < (u & 0xF) + c);
        let c = FlagMode::from(result > 0xFF);

        self.registers.a = result as u8;
        self.cpu_set_flags(z, FlagMode::Set, h, c);
        Ok(0)
    }

    fn process_and(&mut self) -> Result<u32, CpuError> {
        self.registers.a &= (self.fetch_data &0xFF) as u8;
        let z = FlagMode::from(self.registers.a == 0);
        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Set, FlagMode::Clear);
        Ok(0)
    }

    fn process_xor(&mut self) -> Result<u32, CpuError> {
        self.registers.a ^= (self.fetch_data & 0xFF) as u8;

        let z = FlagMode::from(self.registers.a == 0);
        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Clear, FlagMode::Clear);
        Ok(0)
    }

    fn process_or(&mut self) -> Result<u32, CpuError> {
        self.registers.a |= (self.fetch_data & 0xFF) as u8;
        let z = FlagMode::from(self.registers.a == 0);
        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Clear, FlagMode::Clear);
        Ok(0)
    }

    fn process_cp(&mut self) -> Result<u32, CpuError> {
        let z = FlagMode::from(self.registers.a == (self.fetch_data & 0xFF) as u8);
        let h = FlagMode::from((self.registers.a & 0xF) < (self.fetch_data & 0xF) as u8);
        let c = FlagMode::from(self.registers.a < (self.fetch_data & 0xFF) as u8);

        self.cpu_set_flags(z, FlagMode::Set, h, c);
        Ok(0)
    }

    fn process_cb(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let op = self.fetch_data as u8;
        let cb_op = CbOperation::from_byte(op).ok_or(CpuError::InvalidCb(op))?;
        let reg_value = self.cpu_read_r8(Some(cb_op.reg), bus)?;

        cycles += 1;
        if cb_op.reg == RegType::RtHl {
            cycles += 2;
        }

        match cb_op.cb_type {
            CbType::BIT => {
                let z = FlagMode::from(!util::check_bit(reg_value, cb_op.bit));
                self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Set, FlagMode::None);
            }
            CbType::RST => {
                let new_value = util::clear_bit(reg_value, cb_op.bit);
                self.cpu_write_r8(Some(cb_op.reg), new_value, bus)?;
            }
            CbType::SET => {
                let new_value = util::set_bit(reg_value, cb_op.bit);
                self.cpu_write_r8(Some(cb_op.reg), new_value, bus)?;
            }
        }

        Ok(cycles)
    }


    pub(crate) fn process_instruction(&mut self, bus: &mut BUS) -> Result<u32, CpuError> {
        return match self.current_instruction.type_ {
            InType::InNop { .. } => Ok(0),
            InType::InLd => {
                return self.process_ld(bus);
            }
            InType::InJp => {
                return self.process_jp(bus);
            }
            InType::InDi => {
                return self.process_di(bus);
            }
            InType::InEi => {
                return self.process_ei(bus);
            }
            InType::InLdh => {
                return self.process_ldh(bus);
            }
            InType::InPush => {
                return self.process_push(bus);
            }
            InType::InPop => {
                return self.process_pop(bus);
            }
            InType::InCall => {
                return self.process_call(bus);
            }
            InType::InJr => {
                return self.process_jr(bus);
            }
            InType::InRet => {
                return self.process_ret(bus);
            }
            InType::InReti => {
                return self.process_reti(bus);
            }
            InType::InRst => {
                return self.process_rst(bus);
            }
            InType::InDec => {
                return self.process_dec(bus);
            }
            InType::InInc => {
                return self.process_inc(bus);
            }
            InType::InAdd => {
                return self.process_add(bus);
            }
            InType::InAdc => {
                return self.process_adc();
            }
            InType::InSub => {
                return self.process_sub();
            }
            InType::InSbc => {
                return self.process_sbc();
            }
            InType::InAnd => {
                return self.process_and();
            }
            InType::InOr => {
                return self.process_or();
            }
            InType::InCp => {
                return self.process_cp();
            }
            InType::InXor => {
                return self.process_xor();
            }
            _ => Err(CpuError::InvalidInstruction(self.current_opcode as u32)),
        };
    }
}
