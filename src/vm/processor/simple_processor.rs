use crate::vm::error::SimpletronError;
use crate::vm::loader::ParsedInstruction;
use crate::vm::processor::ProcessorInterface;

pub struct SimpleProcessor {
    pub accumulator: i32,
    pub program_counter: usize,
    pub instruction_register: String,
    pub opcode: u32,
    pub operand: String,
}

impl SimpleProcessor {
    pub fn new() -> Self {
        SimpleProcessor {
            accumulator: 0,
            program_counter: 0,
            instruction_register: "".into(),
            opcode: 0,
            operand: "".into(),
        }
    }
}

impl ProcessorInterface for SimpleProcessor {
    fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    fn dump(&self) {
        println!("REGISTERS: ");
        println!("accumulator: +{:0>4}", self.accumulator);
        println!("program counter: {:0>2}", self.program_counter);
        println!("instruction_register: +{:0>4}", self.instruction_register);
        println!("opereration_code: +{:0>2}", self.opcode);
        println!("operand: +{:0>2}", self.operand);
    }

    fn update_state(&mut self, parsed_instr: &ParsedInstruction) -> Result<(), SimpletronError> {
        self.instruction_register = parsed_instr.data.clone();
        let raw = parsed_instr.data.parse::<u32>()?;
        self.opcode = raw / 100;
        self.operand = (raw % 100).to_string();
        Ok(())
    }

    fn get_acc_value(&self) -> i32 {
        self.accumulator
    }

    fn write_acc(&mut self, value: i32) {
        self.accumulator = value;
    }

    fn set_pc(&mut self, value: usize) -> Result<(), SimpletronError> {
        self.program_counter = value;
        Ok(())
    }

    fn get_pc(&self) -> usize {
        self.program_counter
    }
}
