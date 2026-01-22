use crate::assembler::instruction::{AsmInstruction, Mnemonic, Operand};
use crate::vm::error::SimpletronError;

pub struct MnemonicParser;

#[derive(Debug, Clone)]
pub enum ParsedLine {
    Label(String),
    Variable(String),
    Instruction(AsmInstruction),
}

impl MnemonicParser {
    pub fn parse_line(line: &str) -> Result<Option<ParsedLine>, SimpletronError> {
        let code = line.split(';').next().unwrap().trim();
        if code.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = code.split_whitespace().collect();

        // 1️⃣ LABEL (highest priority)
        if code.ends_with(':') {
            let name = code.trim_end_matches(':').to_string();
            return Ok(Some(ParsedLine::Label(name)));
        }

        // 2️⃣ VAR declaration (BEFORE mnemonic parsing)
        if parts.len() == 2 && parts[0] == "VAR" {
            return Ok(Some(ParsedLine::Variable(parts[1].to_string())));
        }

        // 3️⃣ REAL instruction parsing starts here
        let mnemonic = match parts[0] {
            "READ" => Mnemonic::Read,
            "READI" => Mnemonic::ReadI,
            "WRITE" => Mnemonic::Write,
            "WRITEA" => Mnemonic::WriteAcc,

            "LOADM" => Mnemonic::LoadM,
            "LOADI" => Mnemonic::LoadI,
            "STORE" => Mnemonic::Store,

            "ADDM" => Mnemonic::AddM,
            "ADDI" => Mnemonic::AddI,
            "SUBM" => Mnemonic::SubM,
            "SUBI" => Mnemonic::SubI,
            "DIVM" => Mnemonic::DivM,
            "DIVI" => Mnemonic::DivI,
            "MODM" => Mnemonic::ModM,
            "MODI" => Mnemonic::ModI,
            "MULM" => Mnemonic::MulM,
            "MULI" => Mnemonic::MulI,

            "JMP" => Mnemonic::Jump,
            "JZ" => Mnemonic::JumpIfZero,
            "JN" => Mnemonic::JumpIfNegative,
            "JNZ" => Mnemonic::JumpIfNotZero,
            "JG" => Mnemonic::JumpIfGreaterThanZero,

            "HALT" => Mnemonic::Halt,

            _ => return Err(SimpletronError::InvalidInstruction(parts[0].to_string())),
        };

        // operand handling (as you already fixed)
        let operand = match mnemonic {
            Mnemonic::Halt | Mnemonic::WriteAcc => {
                if parts.len() != 1 {
                    return Err(SimpletronError::InvalidInstructionLine);
                }
                None
            }

            Mnemonic::Jump
            | Mnemonic::JumpIfZero
            | Mnemonic::JumpIfNegative
            | Mnemonic::JumpIfNotZero
            | Mnemonic::JumpIfGreaterThanZero => {
                if parts.len() != 2 {
                    return Err(SimpletronError::InvalidInstructionLine);
                }
                let raw = parts[1];
                Some(if raw.chars().all(|c| c.is_ascii_digit()) {
                    Operand::Immediate(raw.parse().unwrap())
                } else {
                    Operand::Label(raw.to_string())
                })
            }

            _ => {
                if parts.len() != 2 {
                    return Err(SimpletronError::InvalidInstructionLine);
                }
                let raw = parts[1];
                Some(if raw.chars().all(|c| c.is_ascii_digit()) {
                    Operand::Immediate(raw.parse().unwrap())
                } else {
                    Operand::Variable(raw.to_string())
                })
            }
        };

        Ok(Some(ParsedLine::Instruction(AsmInstruction {
            mnemonic,
            operand,
        })))
    }
}
