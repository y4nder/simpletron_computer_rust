use std::collections::HashMap;

use crate::{
    assembler::{
        instruction::Operand,
        parser::mnemonic_parser::{MnemonicParser, ParsedLine},
    },
    vm::error::SimpletronError,
};

pub mod encoder;
pub mod instruction;
pub mod parser;
pub mod symbol_table;

type LabelTable = HashMap<String, usize>;
type VarTable = HashMap<String, usize>;

pub fn assemble(source: &str) -> Result<Vec<u16>, SimpletronError> {
    let mut parsed = Vec::new();

    for line in source.lines() {
        if let Some(p) = MnemonicParser::parse_line(line)? {
            parsed.push(p);
        }
    }

    let (labels, vars, _) = first_pass(&parsed)?;
    second_pass(&parsed, &labels, &vars)
}

fn first_pass(lines: &[ParsedLine]) -> Result<(LabelTable, VarTable, usize), SimpletronError> {
    let mut labels = HashMap::new();
    let mut vars = HashMap::new();

    let mut pc = 0;

    // First: count instructions & labels
    for line in lines {
        match line {
            ParsedLine::Label(name) => {
                if labels.contains_key(name) {
                    return Err(SimpletronError::DuplicateLabel(name.clone()));
                }
                labels.insert(name.clone(), pc);
            }
            ParsedLine::Instruction(_) => pc += 1,
            ParsedLine::Variable(_) => {}
        }
    }

    let mut data_addr = pc;

    // Second: allocate variables
    for line in lines {
        if let ParsedLine::Variable(name) = line {
            if vars.contains_key(name) {
                return Err(SimpletronError::DuplicateVariable(name.clone()));
            }
            vars.insert(name.clone(), data_addr);
            data_addr += 1;
        }
    }

    Ok((labels, vars, pc))
}

fn second_pass(
    lines: &[ParsedLine],
    labels: &LabelTable,
    vars: &VarTable,
) -> Result<Vec<u16>, SimpletronError> {
    let mut output = Vec::new();

    for line in lines.iter().cloned() {
        if let ParsedLine::Instruction(mut instr) = line {
            match &instr.operand {
                Some(Operand::Label(name)) => {
                    let addr = labels
                        .get(name)
                        .ok_or(SimpletronError::UnknownLabel(name.clone()))?;
                    instr.operand = Some(Operand::Immediate(*addr));
                }
                Some(Operand::Variable(name)) => {
                    let addr = vars
                        .get(name)
                        .ok_or(SimpletronError::UnknownVariable(name.clone()))?;
                    instr.operand = Some(Operand::Immediate(*addr));
                }
                _ => {}
            }

            output.push(encoder::encode(&instr)?);
        }
    }

    Ok(output)
}
