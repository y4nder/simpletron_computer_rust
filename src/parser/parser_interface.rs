use crate::{Instruction, error::SimpletronError};

pub trait ParserInterface {
    fn parse(&self, file_address: String) -> Result<Vec<Instruction>, SimpletronError>;
}
