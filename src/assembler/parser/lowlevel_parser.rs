use crate::vm::error::SimpletronError;
use crate::{assembler::parser::ParserInterface, vm::loader::ParsedInstruction};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct LowLevelParser {
    pub debug: bool,
}

impl ParserInterface for LowLevelParser {
    fn parse(&self, path: String) -> Result<Vec<ParsedInstruction>, SimpletronError> {
        let reader = Box::new(BufReader::new(File::open(path)?));
        self.start_parse(reader)
    }
}

impl LowLevelParser {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    fn start_parse<R: BufRead>(
        &self,
        reader: R,
    ) -> Result<Vec<ParsedInstruction>, SimpletronError> {
        if self.debug {
            println!("{}", "-".repeat(50));
            println!("Parsing file\n");
        }

        let mut instructions = Vec::new();

        for (line_no, line) in reader.lines().enumerate() {
            let line = line?;

            // Remove comments
            let code_part = line.split(';').next().unwrap().trim();

            // Skip empty or comment-only lines
            if code_part.is_empty() {
                continue;
            }

            // Split by whitespace (tabs OR spaces)
            let parts: Vec<&str> = code_part.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(SimpletronError::InvalidInstructionLine { line: line_no + 1 });
            }

            let address = parts[0]
                .parse::<usize>()
                .map_err(|_| SimpletronError::InvalidAddress { line: line_no + 1 })?;

            let instruction = ParsedInstruction {
                address,
                data: parts[1].to_string(),
            };

            if self.debug {
                println!("{:?}", instruction);
            }

            instructions.push(instruction);
        }

        Ok(instructions)
    }
}
