pub struct MemoryPayload {
    pub address: usize,
    pub data: MemoryData,
}

pub struct MemoryData {
    pub value: String,
}

impl From<u32> for MemoryData {
    fn from(value: u32) -> Self {
        MemoryData {
            value: value.to_string(),
        }
    }
}
