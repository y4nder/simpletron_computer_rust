use crate::{
    assembler::instruction::{AsmInstruction, Mnemonic, Operand},
    vm::error::SimpletronError,
};

pub fn encode(instr: &AsmInstruction) -> Result<u16, SimpletronError> {
    let opcode: u16 = match instr.mnemonic {
        // I/O
        Mnemonic::Read => 10,
        Mnemonic::Write => 11,
        Mnemonic::WriteAcc => 12,
        Mnemonic::ReadI => 13,

        // Memory
        Mnemonic::LoadM => 20,
        Mnemonic::Store => 21,
        Mnemonic::LoadI => 22,

        // Arithmetic (memory)
        Mnemonic::AddM => 30,
        Mnemonic::SubM => 31,
        Mnemonic::DivM => 32,
        Mnemonic::ModM => 33,
        Mnemonic::MulM => 34,

        // Arithmetic (immediate)
        Mnemonic::AddI => 35,
        Mnemonic::SubI => 36,
        Mnemonic::DivI => 37,
        Mnemonic::ModI => 38,
        Mnemonic::MulI => 39,

        // Control flow
        Mnemonic::Jump => 40,
        Mnemonic::JumpIfNegative => 41,
        Mnemonic::JumpIfZero => 42,

        Mnemonic::Halt => 43,
    };

    match &instr.operand {
        Some(Operand::Immediate(value)) => Ok(opcode * 100 + *value as u16),
        Some(Operand::Label(_)) => {
            // This should NEVER reach encoder if second pass worked
            Err(SimpletronError::UnresolvedLabel)
        }
        None => Ok(opcode * 100),
    }
}
