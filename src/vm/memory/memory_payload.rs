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

impl MemoryPayload {
    pub fn new(address: usize, word: &u16) -> Self {
        MemoryPayload {
            address,
            data: MemoryData::new(word),
        }
    }
}

impl MemoryData {
    pub fn new(word: &u16) -> Self {
        MemoryData {
            value: word.to_string(),
        }
    }
}
