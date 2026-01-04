use crate::vm::operation::Opcode;

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: usize,
}
