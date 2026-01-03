use core::fmt;
use std::io;

#[derive(Debug)]
pub enum SimpletronError {
    StoreDataError(String),
    InvalidAddressError(String),
    InvalidInstructionLine { line: usize },
    Io(io::Error),
    InvalidAddress { line: usize },
    InvalidOpcode(u8),
    InvalidReadInput(String),
    DivisionByZero,
    InvalidMemoryData(usize),
}

impl fmt::Display for SimpletronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpletronError::StoreDataError(err) => write!(f, "{}", err),
            SimpletronError::InvalidAddressError(invalid_address) => {
                write!(f, "{} is an invalid address", invalid_address)
            }
            SimpletronError::Io(error) => write!(f, "I/O error {}", error),
            SimpletronError::InvalidInstructionLine { line } => {
                write!(f, "instruction at line {} was invalid", line)
            }
            SimpletronError::InvalidAddress { line } => {
                write!(f, "address at line {} was invalid", line)
            }
            SimpletronError::InvalidOpcode(code) => write!(f, "{} is an invalid opcode", code),
            SimpletronError::InvalidReadInput(err) => write!(f, "{} is an invalid input", err),
            SimpletronError::DivisionByZero => write!(f, "division by zero error"),
            SimpletronError::InvalidMemoryData(address) => {
                write!(f, "invalid memory data at address: {}", address)
            }
        }
    }
}

impl From<std::io::Error> for SimpletronError {
    fn from(value: std::io::Error) -> Self {
        SimpletronError::Io(value)
    }
}
