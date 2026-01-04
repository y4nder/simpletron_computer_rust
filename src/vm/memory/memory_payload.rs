pub struct MemoryPayload {
    pub address: usize,
    pub data: MemoryData,
}

pub struct MemoryData {
    pub value: String,
}

impl From<isize> for MemoryData {
    fn from(value: isize) -> Self {
        MemoryData {
            value: value.to_string(),
        }
    }
}
