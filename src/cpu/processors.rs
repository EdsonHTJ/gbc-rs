use crate::bus::BUS_SINGLETON;
use crate::cpu::error::CpuError;
use crate::cpu::flags::FlagMode;
use crate::cpu::CPU;
use crate::instructions::cb::{CbBitOps, CbOperation, CbOps};
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

    fn process_ld(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        if self.dest_is_mem {
            if RegType::is_some_16_bit(self.current_instruction.reg_2) {
                cycles += 1;
                self.cycle(1);
                BUS_SINGLETON.lock().unwrap().write_16(self.mem_dest, self.fetch_data)?;
            } else {
                BUS_SINGLETON.lock().unwrap().write(self.mem_dest, self.fetch_data as u8)?;
            }
            cycles += 1;
            self.cycle(1);

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

            let val_to_write = self
                .read_register_r2()?
                .wrapping_add_signed(i8::from_be_bytes((self.fetch_data as u8).to_be_bytes()) as i16);
            self.write_register_r1(val_to_write)?;

            return Ok(cycles);
        }

        self.write_register(self.current_instruction.reg_1, self.fetch_data)?;
        Ok(cycles)
    }

    fn process_jp(&mut self) -> Result<u32, CpuError> {
        self.goto_addr(self.fetch_data, false)
    }

    fn process_di(&mut self) -> Result<u32, CpuError> {
        self.interrupt_master_enable = false;
        Ok(0)
    }

    fn process_ei(&mut self) -> Result<u32, CpuError> {
        self.enable_ime = true;
        Ok(0)
    }

    fn process_ldh(&mut self) -> Result<u32, CpuError> {
        if self.current_instruction.reg_1 == Some(RegType::RtA) {
            self.write_register(
                self.current_instruction.reg_1,
                BUS_SINGLETON.lock().unwrap().read(0xFF00 | self.fetch_data)? as u16,
            )?;
        } else {
            BUS_SINGLETON.lock().unwrap().write(0xFF00 | self.mem_dest, self.registers.a)?;
        }

        self.cycle(1);
        Ok(1)
    }

    fn process_pop(&mut self) -> Result<u32, CpuError> {
        let data_lo = self.stack_pop()? as u16 & 0xFF;
        self.cycle(1);
        let data_hi = self.stack_pop()? as u16 & 0xFF;
        self.cycle(1);
        let data = (data_hi << 8) | data_lo;
        self.write_register(self.current_instruction.reg_1, data)?;
        if self.current_instruction.reg_1 == Some(RegType::RtAf) {
            self.write_register(self.current_instruction.reg_1, data & 0xFFF0)?;
        }
        Ok(2)
    }

    fn process_push(&mut self) -> Result<u32, CpuError> {
        let data = self.read_register_r1()?;
        let data_hi = (data >> 8) as u8;
        let data_lo = (data & 0xFF) as u8;
        self.cycle(1);
        self.stack_push(data_hi)?;
        self.cycle(1);
        self.stack_push(data_lo)?;
        self.cycle(1);
        Ok(3)
    }

    fn goto_addr(&mut self, addr: u16, push_pc: bool) -> Result<u32, CpuError> {
        if !self.check_condition() {
            return Ok(0);
        }

        let mut cycles = 0;
        if push_pc {
            self.stack_push_16(self.registers.pc)?;
            self.cycle(2);
            cycles += 2;
        }

        self.registers.pc = addr;
        self.cycle(1);
        Ok(cycles + 1)
    }

    fn process_call(&mut self) -> Result<u32, CpuError> {
        self.goto_addr(self.fetch_data, true)
    }

    fn process_jr(&mut self) -> Result<u32, CpuError> {
        let offset = i8::from_be_bytes((self.fetch_data as u8 & 0xFF).to_be_bytes());
        let next_pc = self.registers.pc.wrapping_add_signed(offset as i16);
        self.goto_addr(next_pc, false)
    }

    fn process_ret(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        if self.current_instruction.cond != None {
            cycles += 1;
            self.cycle(1);
        }

        if !self.check_condition() {
            return Ok(cycles);
        }

        let addr = self.stack_pop_16()?;
        self.cycle(2);
        cycles += 2;
        self.registers.pc = addr;
        self.cycle(1);
        cycles += 1;
        Ok(cycles)
    }

    fn process_reti(&mut self) -> Result<u32, CpuError> {
        self.interrupt_master_enable = true;
        self.process_ret()
    }

    fn process_rst(&mut self) -> Result<u32, CpuError> {
        self.goto_addr(self.current_instruction.param as u16, true)
    }

    fn process_inc(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut value = self.read_register_r1()?.wrapping_add(1);
        if RegType::is_some_16_bit(self.current_instruction.reg_1) {
            cycles += 1;
            self.cycle(1);
        }

        if (self.current_instruction.reg_1 == Some(RegType::RtHl))
            && (self.current_instruction.mode == AddrMode::AmMr)
        {
            value = (BUS_SINGLETON.lock().unwrap().read(self.read_register_r1()?)? + 1) as u16;
            value &= 0xFF;
            BUS_SINGLETON.lock().unwrap().write(self.read_register_r1()?, value as u8)?;
        } else {
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

    fn process_dec(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut value = self.read_register_r1()?.wrapping_add_signed(-1);
        if RegType::is_some_16_bit(self.current_instruction.reg_1) {
            cycles += 1;
            self.cycle(1);
        }

        if (self.current_instruction.reg_1 == Some(RegType::RtHl))
            && (self.current_instruction.mode == AddrMode::AmMr)
        {
            value = (BUS_SINGLETON.lock().unwrap().read(self.read_register_r1()?)?.wrapping_add_signed(-1)) as u16;
            value &= 0xFF;
            BUS_SINGLETON.lock().unwrap().write(self.read_register_r1()?, value as u8)?;
        } else {
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

    fn process_add(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let mut val = self.read_register_r1()?.wrapping_add(self.fetch_data);
        let is_16_bit = RegType::is_some_16_bit(self.current_instruction.reg_1);

        if is_16_bit {
            cycles += 1;
            self.cycle(1);
        }

        if self.current_instruction.reg_1 == Some(RegType::RtSp) {
            val = self
                .read_register_r1()?
                .wrapping_add_signed(i16::from_be_bytes(self.fetch_data.to_be_bytes()));
        }

        let z = val & 0xff == 0;
        let h = (self.read_register_r1()? & 0xf).wrapping_add(self.fetch_data & 0xf) > 0xf;
        let c = (self.read_register_r1()? & 0xFF)
            .wrapping_add_signed(i16::from_be_bytes((self.fetch_data & 0xFF).to_be_bytes()))
            > 0xFF;

        let mut z = FlagMode::from(z);
        let mut h = FlagMode::from(h);
        let mut c = FlagMode::from(c);

        if is_16_bit {
            z = FlagMode::None;
            let val_h = self.read_register_r1()? & 0xFFF + self.fetch_data & 0xFFF >= 0x1000;
            h = FlagMode::from(val_h);

            let n = self.read_register_r1()? as u32 + self.fetch_data as u32;
            c = FlagMode::from(n >= 0x10000);
        }

        if self.current_instruction.reg_1 == Some(RegType::RtSp) {
            z = FlagMode::Clear;
            let val_h =
                (self.read_register_r1()? & 0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10;
            h = FlagMode::from(val_h);
            let val_c = (self.read_register_r1()? & 0xFF)
                .wrapping_add_signed(i16::from_be_bytes((self.fetch_data & 0xFF).to_be_bytes()))
                >= 0x100;
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
        self.registers.a = result as u8;
        let z = FlagMode::from(self.registers.a == 0);
        let h = FlagMode::from((a & 0xF) + (u & 0xF) + c > 0xF);
        let c = FlagMode::from(result > 0xFF);

        self.cpu_set_flags(z, FlagMode::Clear, h, c);
        Ok(0)
    }

    fn process_sub(&mut self) -> Result<u32, CpuError> {
        let r1_value = self.read_register_r1()?;
        let fetch_value = -i16::from_be_bytes(self.fetch_data.to_be_bytes());
        let val = r1_value.wrapping_add_signed(fetch_value as i16) as u16;

        let z = FlagMode::from(val == 0);
        let h = FlagMode::from((self.read_register_r1()? & 0xF) < (self.fetch_data & 0xF));
        let c = FlagMode::from(self.fetch_data > self.read_register_r1()?);

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
        self.registers.a &= (self.fetch_data & 0xFF) as u8;
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

    fn process_cb(&mut self) -> Result<u32, CpuError> {
        let mut cycles = 0;
        let op = self.fetch_data as u8;
        let cb_op = CbOperation::from_byte(op).ok_or(CpuError::InvalidCb(op))?;
        let mut reg_value = self.cpu_read_r8(Some(cb_op.reg))?;

        cycles += 1;
        self.cycle(1);
        if cb_op.reg == RegType::RtHl {
            cycles += 2;
            self.cycle(2);
        }

        match cb_op.cb_bit_ops {
            None => {}
            Some(op) => {
                return match op {
                    CbBitOps::BIT => {
                        let z = FlagMode::from(!util::check_bit(reg_value, cb_op.bit));
                        self.cpu_set_flags(z, FlagMode::Clear, FlagMode::Set, FlagMode::None);
                        Ok(cycles)
                    }
                    CbBitOps::RST => {
                        let new_value = util::clear_bit(reg_value, cb_op.bit);
                        self.cpu_write_r8(Some(cb_op.reg), new_value)?;
                        Ok(cycles)
                    }
                    CbBitOps::SET => {
                        let new_value = util::set_bit(reg_value, cb_op.bit);
                        self.cpu_write_r8(Some(cb_op.reg), new_value)?;
                        Ok(cycles)
                    }
                }
            }
        }

        let cb_ops = cb_op.cb_ops.ok_or(CpuError::InvalidCb(op))?;
        match cb_ops {
            CbOps::RLC => {
                let mut set_c = false;
                let mut result = (reg_value << 1) & 0xFF;
                if util::check_bit(reg_value, 7) {
                    set_c = true;
                    result |= 1;
                }

                self.cpu_write_r8(Some(cb_op.reg), result)?;
                self.cpu_set_flags(
                    FlagMode::from(result == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(set_c),
                );
            }
            CbOps::RRC => {
                let old = reg_value;
                reg_value >>= 1;
                reg_value |= (old << 7) & 0xFF;

                self.cpu_write_r8(Some(cb_op.reg), reg_value)?;
                self.cpu_set_flags(
                    FlagMode::from(reg_value == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(old, 0)),
                );
            }
            CbOps::RL => {
                let old = reg_value;
                reg_value <<= 1;
                reg_value |= self.get_c_flag() as u8;

                self.cpu_write_r8(Some(cb_op.reg), reg_value)?;
                self.cpu_set_flags(
                    FlagMode::from(reg_value == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(old, 7)),
                );
            }
            CbOps::RR => {
                let old = reg_value;
                reg_value >>= 1;
                reg_value |= (self.get_c_flag() as u8) << 7;

                self.cpu_write_r8(Some(cb_op.reg), reg_value)?;
                self.cpu_set_flags(
                    FlagMode::from(reg_value == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(old, 0)),
                );
            }
            CbOps::SLA => {
                let old = reg_value;
                reg_value = (reg_value >> 1) | (reg_value & 0x80);
                self.cpu_write_r8(Some(cb_op.reg), reg_value)?;
                self.cpu_set_flags(
                    FlagMode::from(reg_value == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(old, 0)),
                );
            }
            CbOps::SRA => {
                let old = reg_value;
                reg_value = (reg_value >> 1) | (reg_value & 0x80);
                self.cpu_write_r8(Some(cb_op.reg), reg_value)?;
                self.cpu_set_flags(
                    FlagMode::from(reg_value == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(old, 0)),
                );
            }
            CbOps::SWAP => {
                let u = ((reg_value & 0xF) << 4) | (reg_value >> 4);
                self.cpu_write_r8(Some(cb_op.reg), u)?;
                self.cpu_set_flags(
                    FlagMode::from(u == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::Clear,
                );
            }
            CbOps::SRL => {
                let u = reg_value >> 1;
                self.cpu_write_r8(Some(cb_op.reg), u)?;
                self.cpu_set_flags(
                    FlagMode::from(u == 0),
                    FlagMode::Clear,
                    FlagMode::Clear,
                    FlagMode::from(util::check_bit(reg_value, 0)),
                );
            }
        }

        Ok(cycles)
    }

    fn process_rlca(&mut self) -> Result<u32, CpuError> {
        let mut set_c = false;
        let mut result = (self.registers.a << 1) & 0xFF;
        if util::check_bit(self.registers.a, 7) {
            set_c = true;
            result |= 1;
        }

        self.registers.a = result;
        self.cpu_set_flags(
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::from(set_c),
        );
        Ok(0)
    }

    fn process_rrca(&mut self) -> Result<u32, CpuError> {
        let old = self.registers.a & 0x1;
        self.registers.a >>= 1;
        self.registers.a |= (old << 7) & 0xFF;

        self.cpu_set_flags(
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::from(util::check_bit(old, 0)),
        );
        Ok(0)
    }

    fn process_rla(&mut self) -> Result<u32, CpuError> {
        let old = self.registers.a;
        self.registers.a <<= 1;
        self.registers.a |= self.get_c_flag() as u8;

        self.cpu_set_flags(
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::from(util::check_bit(old, 7)),
        );
        Ok(0)
    }

    fn process_rra(&mut self) -> Result<u32, CpuError> {
        let old = self.registers.a;
        self.registers.a >>= 1;
        self.registers.a |= (self.get_c_flag() as u8) << 7;

        self.cpu_set_flags(
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::from(util::check_bit(old, 0)),
        );
        Ok(0)
    }

    fn process_stop(&mut self) -> Result<u32, CpuError> {
        panic!("STOP instruction not implemented");
    }

    fn process_dda(&mut self) -> Result<u32, CpuError> {
        let mut u = 0;
        let mut fc = false;
        if self.get_h_flag() || (!self.get_n_flag() && ((self.registers.a & 0xF) > 9)) {
            u = 0x06;
        }

        if self.get_c_flag() || ((!self.get_n_flag() && self.registers.a > 0x99)) {
            u |= 0x60;
            fc = true;
        }

        if self.get_n_flag() {
            self.registers.a = self.registers.a.wrapping_sub(u);
        } else {
            self.registers.a = self.registers.a.wrapping_add(u);
        }

        self.cpu_set_flags(
            FlagMode::from(self.registers.a == 0),
            FlagMode::None,
            FlagMode::Clear,
            FlagMode::from(fc),
        );
        Ok(0)
    }

    fn process_cpl(&mut self) -> Result<u32, CpuError> {
        self.registers.a = !self.registers.a;
        self.cpu_set_flags(FlagMode::None, FlagMode::Set, FlagMode::Set, FlagMode::None);
        Ok(0)
    }

    fn process_scf(&mut self) -> Result<u32, CpuError> {
        self.cpu_set_flags(
            FlagMode::None,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::Set,
        );
        Ok(0)
    }

    fn process_ccf(&mut self) -> Result<u32, CpuError> {
        let c = !self.get_c_flag();
        self.cpu_set_flags(
            FlagMode::None,
            FlagMode::Clear,
            FlagMode::Clear,
            FlagMode::from(c),
        );
        Ok(0)
    }

    fn process_halt(&mut self) -> Result<u32, CpuError> {
        self.halted = true;
        Ok(0)
    }

    pub(crate) fn process_instruction(&mut self) -> Result<u32, CpuError> {
        return match self.current_instruction.type_ {
            InType::InNop => Ok(0),
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
            InType::InLdh => {
                return self.process_ldh();
            }
            InType::InPush => {
                return self.process_push();
            }
            InType::InPop => {
                return self.process_pop();
            }
            InType::InCall => {
                return self.process_call();
            }
            InType::InJr => {
                return self.process_jr();
            }
            InType::InRet => {
                return self.process_ret();
            }
            InType::InReti => {
                return self.process_reti();
            }
            InType::InRst => {
                return self.process_rst();
            }
            InType::InDec => {
                return self.process_dec();
            }
            InType::InInc => {
                return self.process_inc();
            }
            InType::InAdd => {
                return self.process_add();
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
            InType::InCb => {
                return self.process_cb();
            }
            InType::InRlca => {
                return self.process_rlca();
            }
            InType::InRrca => {
                return self.process_rrca();
            }
            InType::InRla => {
                return self.process_rla();
            }
            InType::InRra => {
                return self.process_rra();
            }
            InType::InHalt => {
                return self.process_halt();
            }
            InType::InStop => {
                return self.process_stop();
            }
            InType::InDaa => {
                return self.process_dda();
            }
            InType::InCpl => {
                return self.process_cpl();
            }
            InType::InScf => {
                return self.process_scf();
            }
            InType::InCcf => {
                return self.process_ccf();
            }
            _ => Err(CpuError::InvalidInstruction(self.current_opcode as u32)),
        };
    }
}
