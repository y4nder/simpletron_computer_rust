#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    // I/O
    Read,
    Write,
    WriteAcc,
    ReadI,

    // Memory
    LoadM,
    Store,
    LoadI,

    // Arithmetic (memory)
    AddM,
    SubM,
    DivM,
    ModM,
    MulM,

    // Arithmetic (immediate)
    AddI,
    SubI,
    DivI,
    ModI,
    MulI,

    // Control flow
    Jump,
    JumpIfNegative,
    JumpIfZero,

    Halt,
}

#[derive(Debug, Clone)]
pub struct AsmInstruction {
    pub mnemonic: Mnemonic,
    pub operand: Option<Operand>,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immediate(usize),
    Label(String),
}
