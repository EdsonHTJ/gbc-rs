macro_rules! create_instructions {
    () => {{
        let mut instructions: [Option<Instruction>; INSTRUCTION_SET_SIZE] =
            [None; INSTRUCTION_SET_SIZE];

        // 0x0
        instructions[0x00] = Some(Instruction {
            type_: InType::InNop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x01] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmD16,
            reg_1: Some(RegType::RtBc),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x02] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtBc),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x03] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtBc),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x04] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtB),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x05] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtB),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x06] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtB),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x07] = Some(Instruction {
            type_: InType::InRlca,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x08] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmA16R,
            reg_1: None,
            reg_2: Some(RegType::RtSp),
            cond: None,
            param: 0,
        });
        instructions[0x09] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtBc),
            cond: None,
            param: 0,
        });
        instructions[0x0A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtBc),
            cond: None,
            param: 0,
        });
        instructions[0x0B] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtBc),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x0C] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtC),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x0D] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtC),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x0E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtC),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x0F] = Some(Instruction {
            type_: InType::InRrca,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x10] = Some(Instruction {
            type_: InType::InStop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x11] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD16,
            reg_1: Some(RegType::RtDe),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x12] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtDe),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x13] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtDe),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x14] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtD),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x15] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtD),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x16] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtD),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x17] = Some(Instruction {
            type_: InType::InRla,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x18] = Some(Instruction {
            type_: InType::InJr,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        }); // Note: JR D8 is unconditional but could have conditions in other contexts.
        instructions[0x19] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtDe),
            cond: None,
            param: 0,
        });
        instructions[0x1A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtDe),
            cond: None,
            param: 0,
        });
        instructions[0x1B] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtDe),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x1C] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtE),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x1D] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtE),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x1E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtE),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x1F] = Some(Instruction {
            type_: InType::InRra,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x20] = Some(Instruction {
            type_: InType::InJr,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNz),
            param: 0,
        });
        instructions[0x21] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD16,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x22] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmHliR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x23] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x24] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtH),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x25] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtH),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x26] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtH),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x27] = Some(Instruction {
            type_: InType::InDaa,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x28] = Some(Instruction {
            type_: InType::InJr,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtZ),
            param: 0,
        });
        instructions[0x29] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x2A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRHli,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x2B] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x2C] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtL),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x2D] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtL),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x2E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtL),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x2F] = Some(Instruction {
            type_: InType::InCpl,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x30] = Some(Instruction {
            type_: InType::InJr,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNc),
            param: 0,
        });
        instructions[0x31] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD16,
            reg_1: Some(RegType::RtSp),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x32] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmHldR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x33] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtSp),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x34] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmMr,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x35] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmMr,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x36] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMrD8,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x37] = Some(Instruction {
            type_: InType::InScf,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x38] = Some(Instruction {
            type_: InType::InJr,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtC),
            param: 0,
        });
        instructions[0x39] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtSp),
            cond: None,
            param: 0,
        });
        instructions[0x3A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRHld,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x3B] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtSp),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x3C] = Some(Instruction {
            type_: InType::InInc,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x3D] = Some(Instruction {
            type_: InType::InDec,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x3E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x3F] = Some(Instruction {
            type_: InType::InCcf,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x40] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x41] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x42] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x43] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x44] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x45] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x46] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x47] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtB),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x48] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x49] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x4A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x4B] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x4C] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x4D] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x4E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x4F] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x50] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x51] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x52] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x53] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x54] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x55] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x56] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x57] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtD),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });

        instructions[0x58] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x59] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x5A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x5B] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x5C] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x5D] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x5E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x5F] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtE),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x60] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x61] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x62] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x63] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x64] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x65] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x66] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x67] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtH),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x68] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x69] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x6A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x6B] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x6C] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x6D] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x6E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x6F] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtL),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x70] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x71] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x72] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x73] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x74] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x75] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x76] = Some(Instruction {
            type_: InType::InHalt,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0x77] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x78] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x79] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x7A] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x7B] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x7C] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x7D] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x7E] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x7F] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x80] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x81] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x82] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x83] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x84] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x85] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x86] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x87] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0x88] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x89] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x8A] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x8B] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x8C] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x8D] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x8E] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x8F] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });

        instructions[0x90] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x91] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x92] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x93] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x94] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x95] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x96] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x97] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });

        instructions[0x98] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0x99] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0x9A] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0x9B] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0x9C] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0x9D] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0x9E] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0x9F] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0xA0] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0xA1] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0xA2] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0xA3] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0xA4] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0xA5] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0xA6] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0xA7] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });

        instructions[0xA8] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0xA9] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0xAA] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0xAB] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0xAC] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0xAD] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0xAE] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0xAF] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0xB0] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0xB1] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0xB2] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0xB3] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0xB4] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0xB5] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0xB6] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0xB7] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });

        instructions[0xB8] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtB),
            cond: None,
            param: 0,
        });
        instructions[0xB9] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0xBA] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtD),
            cond: None,
            param: 0,
        });
        instructions[0xBB] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtE),
            cond: None,
            param: 0,
        });
        instructions[0xBC] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtH),
            cond: None,
            param: 0,
        });
        instructions[0xBD] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtL),
            cond: None,
            param: 0,
        });
        instructions[0xBE] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0xBF] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0xC0] = Some(Instruction {
            type_: InType::InRet,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNz),
            param: 0,
        });
        instructions[0xC1] = Some(Instruction {
            type_: InType::InPop,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtBc),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xC2] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNz),
            param: 0,
        });
        instructions[0xC3] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xC4] = Some(Instruction {
            type_: InType::InCall,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNz),
            param: 0,
        });
        instructions[0xC5] = Some(Instruction {
            type_: InType::InPush,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtBc),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xC6] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xC7] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x00,
        });
        instructions[0xC8] = Some(Instruction {
            type_: InType::InRet,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtZ),
            param: 0,
        });
        instructions[0xC9] = Some(Instruction {
            type_: InType::InRet,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xCA] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtZ),
            param: 0,
        });
        instructions[0xCB] = Some(Instruction {
            type_: InType::InCb,
            mode: AddrMode::AmD8,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xCC] = Some(Instruction {
            type_: InType::InCall,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtZ),
            param: 0,
        });
        instructions[0xCD] = Some(Instruction {
            type_: InType::InCall,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xCE] = Some(Instruction {
            type_: InType::InAdc,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xCF] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x08,
        });
        instructions[0xD0] = Some(Instruction {
            type_: InType::InRet,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNc),
            param: 0,
        });
        instructions[0xD1] = Some(Instruction {
            type_: InType::InPop,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtDe),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xD2] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNc),
            param: 0,
        });
        instructions[0xD4] = Some(Instruction {
            type_: InType::InCall,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtNc),
            param: 0,
        });
        instructions[0xD5] = Some(Instruction {
            type_: InType::InPush,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtDe),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xD6] = Some(Instruction {
            type_: InType::InSub,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xD7] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x10,
        });
        instructions[0xD8] = Some(Instruction {
            type_: InType::InRet,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtC),
            param: 0,
        });
        instructions[0xD9] = Some(Instruction {
            type_: InType::InReti,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xDA] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtC),
            param: 0,
        });
        // 0xDB is not used in the standard GB/Z80 instruction set.
        instructions[0xDC] = Some(Instruction {
            type_: InType::InCall,
            mode: AddrMode::AmD16,
            reg_1: None,
            reg_2: None,
            cond: Some(CondType::CtC),
            param: 0,
        });
        // 0xDD is not used in the standard GB/Z80 instruction set.
        instructions[0xDE] = Some(Instruction {
            type_: InType::InSbc,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xDF] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x18,
        });

        instructions[0xE0] = Some(Instruction {
            type_: InType::InLdh,
            mode: AddrMode::AmA8R,
            reg_1: None,
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        instructions[0xE1] = Some(Instruction {
            type_: InType::InPop,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xE2] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmMRR,
            reg_1: Some(RegType::RtC),
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        // 0xE3, 0xE4 are not used in the standard GB/Z80 instruction set.
        instructions[0xE5] = Some(Instruction {
            type_: InType::InPush,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xE6] = Some(Instruction {
            type_: InType::InAnd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xE7] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x20,
        });
        instructions[0xE8] = Some(Instruction {
            type_: InType::InAdd,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtSp),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xE9] = Some(Instruction {
            type_: InType::InJp,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtHl),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xEA] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmA16R,
            reg_1: None,
            reg_2: Some(RegType::RtA),
            cond: None,
            param: 0,
        });
        // 0xEB, 0xEC, 0xED are not used in the standard GB/Z80 instruction set.
        instructions[0xEE] = Some(Instruction {
            type_: InType::InXor,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xEF] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x28,
        });
        instructions[0xF0] = Some(Instruction {
            type_: InType::InLdh,
            mode: AddrMode::AmRA8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xF1] = Some(Instruction {
            type_: InType::InPop,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtAf),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xF2] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRMR,
            reg_1: Some(RegType::RtA),
            reg_2: Some(RegType::RtC),
            cond: None,
            param: 0,
        });
        instructions[0xF3] = Some(Instruction {
            type_: InType::InDi,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xF4] = Some(Instruction {
            type_: InType::InNop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        }); // 0xF4 is not a standard GB/Z80 instruction, represented here as NOP for completeness.
        instructions[0xF5] = Some(Instruction {
            type_: InType::InPush,
            mode: AddrMode::AmR,
            reg_1: Some(RegType::RtAf),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xF6] = Some(Instruction {
            type_: InType::InOr,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xF7] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x30,
        });
        instructions[0xF8] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmHlSpr,
            reg_1: Some(RegType::RtHl),
            reg_2: Some(RegType::RtSp),
            cond: None,
            param: 0,
        });
        instructions[0xF9] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRR,
            reg_1: Some(RegType::RtSp),
            reg_2: Some(RegType::RtHl),
            cond: None,
            param: 0,
        });
        instructions[0xFA] = Some(Instruction {
            type_: InType::InLd,
            mode: AddrMode::AmRA16,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xFB] = Some(Instruction {
            type_: InType::InEi,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xFC] = Some(Instruction {
            type_: InType::InNop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        }); // 0xFC is not a standard GB/Z80 instruction, represented here as NOP for completeness.
        instructions[0xFD] = Some(Instruction {
            type_: InType::InNop,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0,
        }); // 0xFD is not a standard GB/Z80 instruction, represented here as NOP for completeness.
        instructions[0xFE] = Some(Instruction {
            type_: InType::InCp,
            mode: AddrMode::AmRD8,
            reg_1: Some(RegType::RtA),
            reg_2: None,
            cond: None,
            param: 0,
        });
        instructions[0xFF] = Some(Instruction {
            type_: InType::InRst,
            mode: AddrMode::AmImp,
            reg_1: None,
            reg_2: None,
            cond: None,
            param: 0x38,
        });
        instructions
    }};
}

pub(crate) use create_instructions;
