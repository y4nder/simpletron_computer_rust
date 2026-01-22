use crate::{
    assembler::instruction::{AsmInstruction, Mnemonic, Operand},
    vm::error::SimpletronError,
};

pub fn encode(instr: &AsmInstruction) -> Result<u16, SimpletronError> {
    let opcode: u16 = match instr.mnemonic {
        Mnemonic::Read => 10,
        Mnemonic::Write => 11,
        Mnemonic::WriteAcc => 12,
        Mnemonic::ReadI => 13,
        Mnemonic::LoadM => 20,
        Mnemonic::Store => 21,
        Mnemonic::LoadI => 22,
        Mnemonic::AddM => 30,
        Mnemonic::SubM => 31,
        Mnemonic::DivM => 32,
        Mnemonic::ModM => 33,
        Mnemonic::MulM => 34,
        Mnemonic::AddI => 35,
        Mnemonic::SubI => 36,
        Mnemonic::DivI => 37,
        Mnemonic::ModI => 38,
        Mnemonic::MulI => 39,
        Mnemonic::Jump => 40,
        Mnemonic::JumpIfNegative => 41,
        Mnemonic::JumpIfZero => 42,
        Mnemonic::Halt => 43,
        Mnemonic::JumpIfNotZero => 44,
        Mnemonic::JumpIfGreaterThanZero => 45,
    };

    match &instr.operand {
        Some(Operand::Immediate(value)) => Ok(opcode * 100 + *value as u16),

        // These should NEVER reach the encoder if passes are correct
        Some(Operand::Label(_)) => Err(SimpletronError::UnresolvedLabel),
        Some(Operand::Variable(_)) => Err(SimpletronError::UnresolvedVariable),

        None => Ok(opcode * 100),
    }
}
