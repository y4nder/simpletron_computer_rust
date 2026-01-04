use crate::vm::{error::SimpletronError, instruction::Instruction, operation::Opcode};

#[derive(Debug, Clone)]
pub struct ParsedInstruction {
    pub address: usize,
    pub data: String,
}

impl TryFrom<ParsedInstruction> for Instruction {
    type Error = SimpletronError;

    fn try_from(value: ParsedInstruction) -> Result<Self, Self::Error> {
        let raw: i32 = value
            .data
            .parse()
            .map_err(|_| SimpletronError::InvalidInstruction(value.data.clone()))?;

        let opcode = Opcode::try_from(raw / 100)?;
        let operand = (raw % 100) as usize;

        Ok(Self { opcode, operand })
    }
}
