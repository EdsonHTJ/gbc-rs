pub mod cb;
mod r#macro;

use crate::instructions::r#macro::create_instructions;

// Addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AddrMode {
    AmImp = 0,
    AmRD16,
    AmRR,
    AmMRR,
    AmR,
    AmRD8,
    AmRMR,
    AmRHli,
    AmRHld,
    AmHliR,
    AmHldR,
    AmRA8,
    AmA8R,
    AmHlSpr,
    AmD16,
    AmD8,
    AmD16R,
    AmMrD8,
    AmMr,
    AmA16R,
    AmRA16,
}

// Register types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum RegType {
    RtA = 0,
    RtF,
    RtB,
    RtC,
    RtD,
    RtE,
    RtH,
    RtL,
    RtAf,
    RtBc,
    RtDe,
    RtHl,
    RtSp,
    RtPc,
}

impl RegType {
    pub fn is_16_bit(&self) -> bool {
        match self {
            RegType::RtAf
            | RegType::RtBc
            | RegType::RtDe
            | RegType::RtHl
            | RegType::RtSp
            | RegType::RtPc => true,
            _ => false,
        }
    }

    pub fn is_some_16_bit(reg: Option<RegType>) -> bool {
        let reg = match reg {
            Some(r) => r,
            None => return false,
        };

        return reg.is_16_bit();
    }
}

// Instruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InType {
    InNop = 0,
    InLd,
    InInc,
    InDec,
    InRlca,
    InAdd,
    InRrca,
    InStop,
    InRla,
    InJr,
    InRra,
    InDaa,
    InCpl,
    InScf,
    InCcf,
    InHalt,
    InAdc,
    InSub,
    InSbc,
    InAnd,
    InXor,
    InOr,
    InCp,
    InPop,
    InJp,
    InPush,
    InRet,
    InCb,
    InCall,
    InReti,
    InLdh,
    InJphl,
    InDi,
    InEi,
    InRst,
    InErr,
    // CB instructions
    InRlc,
    InRrc,
    InRl,
    InRr,
    InSla,
    InSra,
    InSwap,
    InSrl,
    InBit,
    InRes,
    InSet,
}

// Condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondType {
    CtNz = 0,
    CtZ,
    CtNc,
    CtC,
}

// Instruction structure
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub type_: InType,
    pub mode: AddrMode,
    pub reg_1: Option<RegType>,
    pub reg_2: Option<RegType>,
    pub cond: Option<CondType>,
    pub param: u8,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            type_: InType::InNop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        }
    }

    pub fn by_opcode(opcode: u8) -> Option<Instruction> {
        if opcode as usize >= crate::instructions::INSTRUCTIONS.len() {
            return None;
        }

        crate::instructions::INSTRUCTIONS[opcode as usize]
    }
}

const INSTRUCTION_SET_SIZE: usize = 0x100;

const INSTRUCTIONS: [Option<Instruction>; INSTRUCTION_SET_SIZE] = create_instructions!();
