use crate::ParsedInstruction;
use crate::processor::processor_interface::ProcessorInterface;

pub struct SimpleProcessor {
    pub accumulator: u32,
    pub program_counter: u32,
    pub instruction_register: String,
    pub opcode: u32,
    pub operand: String,
}

impl ProcessorInterface for SimpleProcessor {
    fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    fn dump(&self) {
        println!("processor dump {}", self.program_counter);
    }

    fn update_state(&mut self, instruction: ParsedInstruction) {
        self.instruction_register = instruction.data.clone();
        let data: u32 = instruction.data.parse().expect("Invalid instruction");
        self.opcode = data / 100;
        self.operand = instruction.address.to_string();
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
}
