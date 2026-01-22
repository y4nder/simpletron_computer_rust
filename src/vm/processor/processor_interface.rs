use crate::vm::{error::SimpletronError, loader::ParsedInstruction};

pub trait ProcessorInterface {
    fn increment_pc(&mut self);
    fn dump(&self);
    fn update_state(&mut self, instruction: &ParsedInstruction) -> Result<(), SimpletronError>;
    fn get_acc_value(&self) -> i32;
    fn write_acc(&mut self, value: i32);
    fn set_pc(&mut self, value: usize) -> Result<(), SimpletronError>;
    fn get_pc(&self) -> usize;
}
