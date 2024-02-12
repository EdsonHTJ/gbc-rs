use crate::instructions::RegType;


pub enum CbType {
    BIT,
    RST,
    SET,
}

pub struct CbOperation {
    pub cb_type: CbType,
    pub bit: u8,
    pub reg: RegType,
}

impl CbOperation {
    pub fn from_byte(byte: u8) -> Option<CbOperation> {
        let cb_type = match (byte >> 6) & 0x3 {
            0 => CbType::BIT,
            1 => CbType::RST,
            2 => CbType::SET,
            _ => return None,
        };

        let bit = (byte >> 3) & 0x7;
        let reg = RegType::from_cb_u8(byte & 0x7);
        if reg.is_none() {
            return None;
        }

        Some(CbOperation {
            cb_type,
            bit,
            reg: reg.unwrap()
        })
    }
}

impl RegType {
    pub fn from_cb_u8(reg: u8) -> Option<Self> {
        match reg {
            0 => Some(RegType::RtB),
            1 => Some(RegType::RtC),
            2 => Some(RegType::RtD),
            3 => Some(RegType::RtE),
            4 => Some(RegType::RtH),
            5 => Some(RegType::RtL),
            6 => Some(RegType::RtHl),
            7 => Some(RegType::RtA),
            _ => None,
        }
    }
}



