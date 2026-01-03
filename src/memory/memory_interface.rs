use crate::error::SimpletronError;
use crate::memory::memory_payload::MemoryPayload;

pub trait MemoryInterface {
    fn get_memory_length(&self) -> usize;
    fn store_data(&mut self, payload: MemoryPayload) -> Result<bool, SimpletronError>;
    fn read_data(&self, address: usize) -> String;
    fn dump(&self, index: isize);
}
