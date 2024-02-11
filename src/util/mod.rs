pub fn modify_bit(val: u8, bit: u8, set: bool) -> u8 {
    if set {
        val | (1 << bit)
    } else {
        val & !(1 << bit)
    }
}

pub fn set_bit(val: u8, bit: u8) -> u8 {
    modify_bit(val, bit, true)
}

pub fn clear_bit(val: u8, bit: u8) -> u8 {
    modify_bit(val, bit, false)
}

pub fn check_bit(val: u8, bit: u8) -> bool {
    (val & (1 << bit)) != 0
}
