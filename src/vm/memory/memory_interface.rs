use crate::vm::error::SimpletronError;
use crate::vm::memory::MemoryPayload;

pub trait MemoryInterface {
    fn get_memory_length(&self) -> usize;
    fn store_data(&mut self, payload: MemoryPayload) -> Result<(), SimpletronError>;
    fn read_data(&self, address: usize) -> Result<String, SimpletronError>;
    fn dump(&self, index: isize);
}
