use crate::error::SimpletronError;
use crate::memory::memory_interface::MemoryInterface;
use crate::memory::memory_payload::MemoryPayload;

const DEFAULT_CELL: &str = "0";

pub struct SingleList {
    pub memory: Vec<String>,
}

impl SingleList {
    pub fn new(size: Option<u16>) -> Self {
        let memory = match size {
            Some(size) => vec![DEFAULT_CELL.to_string(); size as usize],
            None => vec!["0".to_string(); 100 as usize],
        };

        SingleList { memory }
    }
}

impl SingleList {
    fn render_header() {
        print!("     ");
        for i in 0..10 {
            print!("{:>9}", i);
        }
        println!();
    }

    fn render_rows(&self, pointer_index: Option<usize>) {
        let len = self.memory.len();

        for i in (0..len).step_by(10) {
            // Mimic address like "  00", "  10", "  20"
            print!("{:5}0  ", i / 10);

            for j in i..(i + 10) {
                if j < len {
                    let cell = if Some(j) == pointer_index {
                        format!("-> {:0>4}", self.memory[j])
                    } else {
                        format!("+{:0>4}", self.memory[j])
                    };

                    print!("{:>7}  ", cell);
                }
            }
            println!();
        }
    }

    fn is_valid_address(&self, address: usize) -> bool {
        self.memory.len() > address
    }
}

impl MemoryInterface for SingleList {
    fn get_memory_length(&self) -> usize {
        self.memory.len()
    }

    fn store_data(&mut self, payload: MemoryPayload) -> Result<bool, SimpletronError> {
        if payload.address >= self.memory.len() {
            return Err(SimpletronError::InvalidAddressError(
                payload.address.to_string(),
            ));
        }

        self.memory[payload.address] = payload.data.value;
        Ok(true)
    }

    fn read_data(&self, address: usize) -> Result<String, SimpletronError> {
        match self.is_valid_address(address) {
            true => Ok(self.memory[address].clone()),
            false => Err(SimpletronError::InvalidAddressError(address.to_string())),
        }
    }

    fn dump(&self, index: isize) {
        println!("Memory Dump:");

        let pointer_index = if index >= 0 {
            Some(index as usize)
        } else {
            None
        };

        Self::render_header();
        self.render_rows(pointer_index);
    }
}
