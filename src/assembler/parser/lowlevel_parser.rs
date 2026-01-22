use crate::vm::error::SimpletronError;
use crate::{assembler::parser::ParserInterface, vm::loader::ParsedInstruction};

pub struct LowLevelParser {
    pub debug: bool,
}

impl ParserInterface for LowLevelParser {
    fn parse_line(
        line: &str,
        line_no: usize,
    ) -> Result<Option<ParsedInstruction>, SimpletronError> {
        // Strip comments
        let code = line.split(';').next().unwrap().trim();

        // Ignore empty lines
        if code.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = code.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(SimpletronError::InvalidInstructionLine);
        }

        let address = parts[0]
            .parse::<usize>()
            .map_err(|_| SimpletronError::InvalidAddress { line: line_no })?;

        Ok(Some(ParsedInstruction {
            address,
            data: parts[1].to_string(),
        }))
    }
}

impl LowLevelParser {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }
}
