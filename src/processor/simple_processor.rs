use crate::instruction::ParsedInstruction;
use crate::processor::ProcessorInterface;

pub struct SimpleProcessor {
    pub accumulator: u32,
    pub program_counter: u32,
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

    fn update_state(&mut self, parsed_instr: &ParsedInstruction) {
        self.instruction_register = parsed_instr.data.clone();
        let raw: u32 = parsed_instr.data.parse().expect("Invalid instruction");
        self.opcode = raw / 100;
        self.operand = (raw % 100).to_string();
    }

    fn get_acc_value(&self) -> u32 {
        self.accumulator
    }

    fn write_acc(&mut self, value: u32) {
        self.accumulator = value;
    }

    fn set_pc(&mut self, value: usize) {
        self.program_counter = value.try_into().unwrap();
    }

    fn get_pc(&self) -> u32 {
        self.program_counter
    }
}
