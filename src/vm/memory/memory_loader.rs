use crate::vm::error::SimpletronError;
use crate::vm::memory::MemoryInterface;
use crate::vm::memory::{MemoryData, MemoryPayload};

pub struct MemoryLoader<'a, M>
where
    M: MemoryInterface,
{
    memory: &'a mut M,
    debug: bool,
}

impl<'a, M> MemoryLoader<'a, M>
where
    M: MemoryInterface,
{
    pub fn new(memory: &'a mut M, debug: bool) -> Self {
        Self { memory, debug }
    }

    pub fn load_program(&mut self, program: &[u16]) -> Result<(), SimpletronError> {
        if self.debug {
            println!("\nloading program into memory\n");
        }

        for (address, word) in program.iter().enumerate() {
            self.memory.store_data(MemoryPayload {
                address,
                data: MemoryData {
                    value: word.to_string(),
                },
            })?;
        }

        Ok(())
    }
}
