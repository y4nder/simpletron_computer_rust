use crate::vm::error::SimpletronError;
use crate::vm::loader::ParsedInstruction;

pub trait ParserInterface {
    fn parse(&self, file_address: String) -> Result<Vec<ParsedInstruction>, SimpletronError>;
}
