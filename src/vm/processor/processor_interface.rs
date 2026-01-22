use crate::vm::loader::ParsedInstruction;

pub trait ProcessorInterface {
    fn increment_pc(&mut self);
    fn dump(&self);
    fn update_state(&mut self, instruction: &ParsedInstruction);
    fn get_acc_value(&self) -> i32;
    fn write_acc(&mut self, value: i32);
    fn set_pc(&mut self, value: usize);
    fn get_pc(&self) -> u32;
}
