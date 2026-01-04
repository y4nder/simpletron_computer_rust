use crate::{
    assembler::{
        instruction::Operand,
        parser::mnemonic_parser::{MnemonicParser, ParsedLine},
        symbol_table::SymbolTable,
    },
    vm::error::SimpletronError,
};

pub mod encoder;
pub mod instruction;
pub mod parser;
pub mod symbol_table;

pub fn assemble(source: &str) -> Result<Vec<u16>, SimpletronError> {
    let mut parsed = Vec::new();

    for line in source.lines() {
        if let Some(p) = MnemonicParser::parse_line(line)? {
            parsed.push(p);
        }
    }

    let symbols = first_pass(&parsed)?;
    let encoded = second_pass(&parsed, &symbols)?;

    Ok(encoded)
}

fn first_pass(lines: &[ParsedLine]) -> Result<SymbolTable, SimpletronError> {
    let mut table = SymbolTable::new();
    let mut pc = 0;

    for line in lines {
        match line {
            ParsedLine::Label(name) => {
                if table.contains_key(name) {
                    return Err(SimpletronError::DuplicateLabel(name.clone()));
                }
                table.insert(name.clone(), pc);
            }
            ParsedLine::Instruction(_) => {
                pc += 1;
            }
        }
    }

    Ok(table)
}

fn second_pass(lines: &[ParsedLine], symbols: &SymbolTable) -> Result<Vec<u16>, SimpletronError> {
    let mut output = Vec::new();

    for line in lines.iter().cloned() {
        if let ParsedLine::Instruction(mut instr) = line {
            if let Some(Operand::Label(name)) = &instr.operand {
                let addr = symbols
                    .get(name)
                    .ok_or(SimpletronError::UnknownLabel(name.clone()))?;

                instr.operand = Some(Operand::Immediate(*addr));
            }

            output.push(encoder::encode(&instr)?);
        }
    }

    Ok(output)
}
