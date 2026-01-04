pub mod cli;
pub mod controller;
pub mod error;
pub mod instruction;
pub mod memory;
pub mod operation;
pub mod parser;
pub mod processor;

pub use instruction::ParsedInstruction;
pub use memory::single_list::SimpleMemory;
