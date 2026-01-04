use crate::error::SimpletronError;
use crate::memory::memory_interface::MemoryInterface;
use crate::memory::memory_payload::{MemoryData, MemoryPayload};
use crate::parser::ParserInterface;

pub struct MemoryLoader<'a, P, M>
where
    P: ParserInterface,
    M: MemoryInterface,
{
    parser: P,
    memory: &'a mut M,
    debug: bool,
}

impl<'a, P, M> MemoryLoader<'a, P, M>
where
    P: ParserInterface,
    M: MemoryInterface,
{
    pub fn new(parser: P, memory: &'a mut M, debug: bool) -> Self {
        Self {
            parser,
            memory,
            debug,
        }
    }

    pub fn load(&mut self, file_path: String) -> Result<(), SimpletronError> {
        if self.debug {
            println!("\nloading instructions to memory\n");
        }

        let instructions = self.parser.parse(file_path)?;

        instructions
            .into_iter()
            .map(|instr| MemoryPayload {
                address: instr.address,
                data: MemoryData { value: instr.data },
            })
            .try_for_each(|payload| self.memory.store_data(payload))?;

        Ok(())
    }
}
