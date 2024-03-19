use crate::instructions::RegType;

pub enum CbBitOps {
    BIT,
    RST,
    SET,
}

pub enum CbOps {
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
}

pub struct CbOperation {
    pub cb_bit_ops: Option<CbBitOps>,
    pub cb_ops: Option<CbOps>,
    pub bit: u8,
    pub reg: RegType,
}

impl CbOperation {
    pub fn from_byte(byte: u8) -> Option<CbOperation> {
        let cb_bit_ops = match (byte >> 6) & 0x3 {
            1 => Some(CbBitOps::BIT),
            2 => Some(CbBitOps::RST),
            3 => Some(CbBitOps::SET),
            _ => None,
        };

        let bit = (byte >> 3) & 0x7;
        let bit_ops = match bit {
            0 => Some(CbOps::RLC),
            1 => Some(CbOps::RRC),
            2 => Some(CbOps::RL),
            3 => Some(CbOps::RR),
            4 => Some(CbOps::SLA),
            5 => Some(CbOps::SRA),
            6 => Some(CbOps::SWAP),
            7 => Some(CbOps::SRL),
            _ => None,
        };

        let reg = RegType::from_cb_u8(byte & 0x7);
        if reg.is_none() {
            return None;
        }

        Some(CbOperation {
            cb_bit_ops,
            cb_ops: bit_ops,
            bit,
            reg: reg.unwrap(),
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
