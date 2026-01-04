use core::fmt;
use std::io;

#[derive(Debug)]
pub enum SimpletronError {
    StoreDataError(String),
    InvalidAddressError(String),
    InvalidInstructionLine,
    Io(io::Error),
    InvalidAddress { line: usize },
    InvalidOpcode(i32),
    InvalidReadInput(String),
    DivisionByZero,
    InvalidMemoryData(usize),
    InvalidInstruction(String),
    InvalidOperand(String),
    DuplicateLabel(String),
    UnknownLabel(String),
    UnresolvedLabel,
    DuplicateVariable(String),
    UnknownVariable(String),
    UnresolvedVariable,
}

impl fmt::Display for SimpletronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpletronError::StoreDataError(err) => write!(f, "{}", err),
            SimpletronError::InvalidAddressError(invalid_address) => {
                write!(f, "{} is an invalid address", invalid_address)
            }
            SimpletronError::Io(error) => write!(f, "I/O error {}", error),
            SimpletronError::InvalidInstructionLine => {
                write!(f, "instruction at line was invalid")
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
            SimpletronError::InvalidInstruction(error) => {
                write!(f, "{} is an invalid instruction", error)
            }
            SimpletronError::InvalidOperand(error) => {
                write!(f, "{} is an invalid instruction", error)
            }
            SimpletronError::DuplicateLabel(label) => {
                write!(f, "{} deteceted as duplicate label", label)
            }
            SimpletronError::UnknownLabel(label) => write!(f, "Unknown Label {}", label),
            SimpletronError::UnresolvedLabel => write!(f, "Unresolved Label error"),
            SimpletronError::DuplicateVariable(variable) => {
                write!(f, "Duplicate Variable {}", variable)
            }
            SimpletronError::UnknownVariable(variable) => {
                write!(f, "Unknown Variable {}", variable)
            }
            SimpletronError::UnresolvedVariable => write!(f, "Unresolve Variable Error"),
        }
    }
}

impl From<std::io::Error> for SimpletronError {
    fn from(value: std::io::Error) -> Self {
        SimpletronError::Io(value)
    }
}
