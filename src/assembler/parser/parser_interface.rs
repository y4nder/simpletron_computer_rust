use crate::vm::error::SimpletronError;
use crate::vm::loader::ParsedInstruction;

pub trait ParserInterface {
    fn parse_line(line: &str, line_no: usize)
    -> Result<Option<ParsedInstruction>, SimpletronError>;
}
