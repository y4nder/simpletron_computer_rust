use crate::operation::Opcode;

#[derive(Debug, Clone)]
pub struct ParsedInstruction {
    pub address: usize,
    pub data: String,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: usize,
}

impl From<ParsedInstruction> for Instruction {
    fn from(value: ParsedInstruction) -> Self {
        let data: u8 = value.data.parse().expect("Invalid instruction");
        Instruction {
            opcode: Opcode::from_code(data).unwrap(),
            operand: value.address,
        }
    }
}
