use crate::error::SimpletronError;
use crate::memory::memory_interface::MemoryInterface;
use crate::memory::memory_payload::MemoryPayload;

pub struct SingleList {
    pub memory: Vec<String>,
}

impl SingleList {
    pub fn new(size: Option<u16>) -> Self {
        let memory = match size {
            Some(size) => vec!["0000".to_string(); size as usize],
            None => vec!["0000".to_string(); 99 as usize],
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
                        format!("-> {}", self.memory[j])
                    } else {
                        format!("+{}", self.memory[j])
                    };

                    print!("{:>7}  ", cell);
                }
            }
            println!();
        }
    }
}

impl MemoryInterface for SingleList {
    fn get_memory_length(&self) -> usize {
        self.memory.len()
    }

    fn store_data(&mut self, payload: MemoryPayload) -> Result<bool, SimpletronError> {
        self.memory.insert(payload.address, payload.data.value);
        Ok(true)
    }

    fn read_data(&self, address: usize) -> String {
        self.memory[address].clone()
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
