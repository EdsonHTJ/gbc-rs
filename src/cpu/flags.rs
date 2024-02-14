use crate::cpu::CPU;
use crate::util;

pub enum FlagMode {
    None,
    Set,
    Clear,
}

impl From<bool> for FlagMode {
    fn from(value: bool) -> Self {
        if value {
            FlagMode::Set
        } else {
            FlagMode::Clear
        }
    }
}

impl CPU {
    pub(crate) fn get_z_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 7);
    }

    pub(crate) fn get_c_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 4);
    }

    pub(crate) fn get_n_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 5);
    }

    pub(crate) fn get_h_flag(&self) -> bool {
        return util::check_bit(self.registers.f, 6);
    }

    fn set_z_flag(&mut self, value: bool) {
        self.registers.f = util::modify_bit(self.registers.f, 7, value);
    }

    fn set_c_flag(&mut self, value: bool) {
        self.registers.f = util::modify_bit(self.registers.f, 4, value);
    }

    fn set_n_flag(&mut self, value: bool) {
        self.registers.f = util::modify_bit(self.registers.f, 5, value);
    }

    fn set_h_flag(&mut self, value: bool) {
        self.registers.f = util::modify_bit(self.registers.f, 6, value);
    }

    pub(crate) fn cpu_set_flags(&mut self, z: FlagMode, n: FlagMode, h: FlagMode, c: FlagMode) {
        match z {
            FlagMode::Set => self.set_z_flag(true),
            FlagMode::Clear => self.set_z_flag(false),
            _ => {}
        }

        match n {
            FlagMode::Set => self.set_n_flag(true),
            FlagMode::Clear => self.set_n_flag(false),
            _ => {}
        }

        match h {
            FlagMode::Set => self.set_h_flag(true),
            FlagMode::Clear => self.set_h_flag(false),
            _ => {}
        }

        match c {
            FlagMode::Set => self.set_c_flag(true),
            FlagMode::Clear => self.set_c_flag(false),
            _ => {}
        }
    }
}
