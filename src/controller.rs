use crate::{
    error::SimpletronError,
    instruction::Instruction,
    memory::{
        memory_interface::MemoryInterface,
        memory_payload::{MemoryData, MemoryPayload},
    },
    operation::Opcode,
    processor::processor_interface::ProcessorInterface,
};

use std::io::{self, Write};

pub struct Controller<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    cpu: P,
    memory: M,
    debug: bool,
}

impl<P, M> Controller<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    pub fn new(cpu: P, memory: M, debug: bool) -> Self {
        Self { cpu, memory, debug }
    }
}

impl<P, M> Controller<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), SimpletronError> {
        use Opcode::*;

        match instruction.opcode {
            Read => self.read(instruction.operand, self.debug)?,
            Write => self.write(instruction.operand, self.debug)?,
            WriteAcc => self.write_acc(self.debug)?,
            ReadI => self.read_i(self.debug)?,
            LoadM => self.load_m(instruction.operand, self.debug)?,
            Store => self.store(instruction.operand, self.debug)?,
            LoadI => self.load_i(instruction.operand, self.debug)?,
            AddM => self.add_m(instruction.operand, self.debug)?,
            SubM => self.sub_m(instruction.operand, self.debug)?,
            DivM => self.div_m(instruction.operand, self.debug)?,
            ModM => self.mod_m(instruction.operand, self.debug)?,
            MulM => self.mul_m(instruction.operand, self.debug)?,
            AddI => self.add_i(instruction.operand, self.debug)?,
            SubI => self.sub_i(instruction.operand, self.debug)?,
            DivI => self.div_i(instruction.operand, self.debug)?,
            ModI => self.mod_i(instruction.operand, self.debug)?,
            MulI => self.mul_i(instruction.operand, self.debug)?,
            Jump => self.jump(instruction.operand, self.debug)?,
            JumpIfNegative => self.jump_if_negative(instruction.operand, self.debug)?,
            JumpIfZero => self.jump_if_zero(instruction.operand, self.debug)?,
            Halt => self.halt(self.debug)?,
        }
        Ok(())
    }

    fn read(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
            println!(
                "get the value from keyboard and store to address {}",
                address
            );
        }

        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let value: i32 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid number");
                return Err(SimpletronError::InvalidReadInput(input));
            }
        };

        self.memory.store_data(MemoryPayload {
            address,
            data: MemoryData {
                value: value.to_string(),
            },
        })?;
        self.cpu.increment_pc();
        Ok(())
    }

    fn write(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let value = self.memory.read_data(address)?;
        println!("Memory[{}] = {}", address, value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn write_acc(&mut self, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let value = self.cpu.get_acc_value();
        println!("accumulator: {}", value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn read_i(&mut self, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let value: u32 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid number");
                return Err(SimpletronError::InvalidReadInput(input));
            }
        };
        self.cpu.write_acc(value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn load_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }
        let value = self.memory.read_data(address)?;
        self.cpu.write_acc(value.parse().unwrap());
        self.cpu.increment_pc();
        Ok(())
    }

    fn store(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        self.memory.store_data(MemoryPayload {
            address,
            data: MemoryData {
                value: self.cpu.get_acc_value().to_string(),
            },
        })?;

        self.cpu.increment_pc();
        Ok(())
    }

    fn load_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        self.cpu.write_acc(operand.try_into().unwrap());
        self.cpu.increment_pc();
        Ok(())
    }

    fn add_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc + value) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn sub_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc - value) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mul_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc * value) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn div_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let divisor: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        if divisor == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc / divisor) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mod_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let divisor: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        if divisor == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc % divisor) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn add_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc + operand as i32) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn sub_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc - operand as i32) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mul_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc * operand as i32) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn div_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        if operand == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc / operand as i32) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mod_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        if operand == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc % operand as i32) as u32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn jump(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        self.cpu.set_pc(address);
        Ok(())
    }

    fn jump_if_negative(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let acc = self.cpu.get_acc_value() as i32;
        if acc < 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }

    fn jump_if_zero(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
        }

        let acc = self.cpu.get_acc_value();
        if acc == 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }

    fn halt(&mut self, debug: bool) -> Result<(), SimpletronError> {
        if debug {
            println!("{}", "-".repeat(100));
            println!("Program halted");
        }

        self.cpu.dump();
        self.memory.dump(-1);

        std::process::exit(0);
    }
}
