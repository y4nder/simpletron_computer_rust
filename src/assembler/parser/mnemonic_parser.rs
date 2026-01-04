use crate::assembler::instruction::{AsmInstruction, Mnemonic, Operand};
use crate::vm::error::SimpletronError;

pub struct MnemonicParser;

#[derive(Debug, Clone)]
pub enum ParsedLine {
    Label(String),
    Instruction(AsmInstruction),
}

impl MnemonicParser {
    pub fn parse_line(line: &str) -> Result<Option<ParsedLine>, SimpletronError> {
        let code = line.split(';').next().unwrap().trim();
        if code.is_empty() {
            return Ok(None);
        }

        // Label definition
        if code.ends_with(':') {
            let name = code.trim_end_matches(':').to_string();
            return Ok(Some(ParsedLine::Label(name)));
        }

        let parts: Vec<&str> = code.split_whitespace().collect();
        let head = parts[0];

        let mnemonic = match head {
            // I/O
            "READ" => Mnemonic::Read,
            "READI" => Mnemonic::ReadI,
            "WRITE" => Mnemonic::Write,
            "WRITEA" => Mnemonic::WriteAcc,

            // Memory
            "LOADM" => Mnemonic::LoadM,
            "LOADI" => Mnemonic::LoadI,
            "STORE" => Mnemonic::Store,

            // Arithmetic (memory)
            "ADDM" => Mnemonic::AddM,
            "SUBM" => Mnemonic::SubM,
            "DIVM" => Mnemonic::DivM,
            "MODM" => Mnemonic::ModM,
            "MULM" => Mnemonic::MulM,

            // Arithmetic (immediate)
            "ADDI" => Mnemonic::AddI,
            "SUBI" => Mnemonic::SubI,
            "DIVI" => Mnemonic::DivI,
            "MODI" => Mnemonic::ModI,
            "MULI" => Mnemonic::MulI,

            // Control flow
            "JMP" => Mnemonic::Jump,
            "JN" => Mnemonic::JumpIfNegative,
            "JZ" => Mnemonic::JumpIfZero,

            // Halt
            "HALT" => Mnemonic::Halt,

            _ => return Err(SimpletronError::InvalidInstruction(head.to_string())),
        };

        // Operand rules:
        // - HALT, WRITEA: no operand
        // - Everything else here requires exactly one operand (including jumps)
        let operand = match mnemonic {
            Mnemonic::Halt | Mnemonic::WriteAcc => None,
            _ => {
                let raw = parts
                    .get(1)
                    .ok_or(SimpletronError::InvalidInstructionLine { line: parts.len() })?;

                if raw.chars().all(|c| c.is_ascii_digit()) {
                    Some(Operand::Immediate(raw.parse().unwrap()))
                } else {
                    Some(Operand::Label(raw.to_string()))
                }
            }
        };

        Ok(Some(ParsedLine::Instruction(AsmInstruction {
            mnemonic,
            operand,
        })))
    }
}
