

// Addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegType {
    RtNone = 0,
    RtA,
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

// Instruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InType {
    InNone = 0,
    InNop,
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
    CtNone = 0,
    CtNz,
    CtZ,
    CtNc,
    CtC,
}

// Instruction structure
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub type_: InType,
    pub mode: AddrMode,
    pub reg_1: RegType,
    pub reg_2: RegType,
    pub cond: CondType,
    pub param: u8,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            type_: InType::InNone,
            mode: AddrMode::AmImp,
            reg_1: RegType::RtNone,
            reg_2: RegType::RtNone,
            cond: CondType::CtNone,
            param: 0,
        }
    }

    pub fn by_opcode(opcode: u8) -> Instruction {
        if opcode as usize >= INSTRUCTIONS.len() {
            return Instruction::new();
        }

        INSTRUCTIONS[opcode as usize].unwrap()
    }
}

const INSTRUCTION_SET_SIZE: usize = 0x100;

macro_rules! create_instructions {
    () => {{
        let mut instructions: [Option<Instruction>; INSTRUCTION_SET_SIZE] = [None; INSTRUCTION_SET_SIZE];

        instructions[0x00] = Some(Instruction { type_: InType::InNop, mode: AddrMode::AmImp,  reg_1: RegType::RtNone, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions[0x05] = Some(Instruction { type_: InType::InDec, mode: AddrMode::AmR, reg_1: RegType::RtB, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions[0x0E] = Some(Instruction { type_: InType::InLd, mode: AddrMode::AmRD8, reg_1: RegType::RtC, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions[0xAF] = Some(Instruction { type_: InType::InXor, mode: AddrMode::AmR, reg_1: RegType::RtA, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions[0xC3] = Some(Instruction { type_: InType::InJp, mode: AddrMode::AmD16, reg_1: RegType::RtNone, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions[0xF3] = Some(Instruction { type_: InType::InDi, mode: AddrMode::AmImp,  reg_1: RegType::RtNone, reg_2: RegType::RtNone, cond: CondType::CtNone, param: 0});
        instructions
    }};
}

const INSTRUCTIONS: [Option<Instruction>; INSTRUCTION_SET_SIZE] = create_instructions!();
