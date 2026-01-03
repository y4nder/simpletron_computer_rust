use crate::{ParsedInstruction, error::SimpletronError};

pub trait ParserInterface {
    fn parse(&self, file_address: String) -> Result<Vec<ParsedInstruction>, SimpletronError>;
}
