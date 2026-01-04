use crate::vm::error::SimpletronError;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Read,
    Write,
    WriteAcc,
    ReadI,
    LoadM,
    Store,
    LoadI,
    AddM,
    SubM,
    DivM,
    ModM,
    MulM,
    AddI,
    SubI,
    DivI,
    ModI,
    MulI,
    Jump,
    JumpIfNegative,
    JumpIfZero,
    Halt,
}

impl TryFrom<i32> for Opcode {
    type Error = SimpletronError;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        use Opcode::*;

        match code {
            10 => Ok(Read),
            11 => Ok(Write),
            12 => Ok(WriteAcc),
            13 => Ok(ReadI),
            20 => Ok(LoadM),
            21 => Ok(Store),
            22 => Ok(LoadI),
            30 => Ok(AddM),
            31 => Ok(SubM),
            32 => Ok(DivM),
            33 => Ok(ModM),
            34 => Ok(MulM),
            35 => Ok(AddI),
            36 => Ok(SubI),
            37 => Ok(DivI),
            38 => Ok(ModI),
            39 => Ok(MulI),
            40 => Ok(Jump),
            41 => Ok(JumpIfNegative),
            42 => Ok(JumpIfZero),
            43 => Ok(Halt),
            _ => Err(SimpletronError::InvalidOpcode(code)),
        }
    }
}
