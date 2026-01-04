use crate::{error::SimpletronError, instruction::ParsedInstruction};

pub trait ParserInterface {
    fn parse(&self, file_address: String) -> Result<Vec<ParsedInstruction>, SimpletronError>;
}
