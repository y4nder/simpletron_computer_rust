use crate::error::SimpletronError;
use crate::memory::memory_interface::MemoryInterface;
use crate::memory::memory_payload::{MemoryData, MemoryPayload};
use crate::parser::ParserInterface;

pub struct MemoryLoader<P, M>
where
    P: ParserInterface,
    M: MemoryInterface,
{
    parser: P,
    memory: M,
    debug: bool,
}

impl<P, M> MemoryLoader<P, M>
where
    P: ParserInterface,
    M: MemoryInterface,
{
    pub fn new(parser: P, memory: M, debug: bool) -> Self {
        Self {
            parser,
            memory,
            debug,
        }
    }

    pub fn memory(&self) -> &M {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut M {
        &mut self.memory
    }

    pub fn load(&mut self, file_path: String) -> Result<(), SimpletronError> {
        if self.debug {
            println!("\nloading instructions to memory\n");
        }

        let instructions = self.parser.parse(file_path)?;

        for instruction in instructions {
            let payload = MemoryPayload {
                address: instruction.address,
                data: MemoryData {
                    value: instruction.data,
                },
            };

            self.memory.store_data(payload)?;
        }

        Ok(())
    }
}
