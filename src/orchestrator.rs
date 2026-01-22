use crate::vm::error::SimpletronError;
use crate::vm::instruction::Instruction;
use crate::vm::loader::ParsedInstruction;
use crate::vm::memory::{MemoryData, MemoryInterface, MemoryPayload};
use crate::vm::processor::ProcessorInterface;

use std::io::{self, Write};

pub struct Orchestrator<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    cpu: P,
    memory: M,
    debug: bool,
}

impl<P, M> Orchestrator<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    pub fn new(cpu: P, memory: M, debug: bool) -> Self {
        Self { cpu, memory, debug }
    }
}

impl<P, M> Orchestrator<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    pub fn run(&mut self) -> Result<(), SimpletronError> {
        println!("*** Welcome to Simpletron ***");
        println!("*** Program Loaded Succesfully ***\n");
        loop {
            let (address, data) = self.fetch_instruction()?;
            let parsed_instr = ParsedInstruction { address, data };
            self.cpu.update_state(&parsed_instr);

            if self.debug {
                println!("");
                self.cpu.dump();
                self.memory.dump(self.cpu.get_pc().try_into().unwrap());
            }
            self.execute(Instruction::try_from(parsed_instr)?)?;

            if self.debug {
                wait_for_keypress();
            }
        }

        fn wait_for_keypress() {
            println!("\nPress Enter to continue...");
            let mut buf = String::new();
            let _ = io::stdin().read_line(&mut buf);
        }
    }

    fn fetch_instruction(&self) -> Result<(usize, String), SimpletronError> {
        let address = usize::try_from(self.cpu.get_pc()).unwrap();
        let data = self.memory.read_data(address)?;
        Ok((address, data))
    }

    pub fn execute(&mut self, instr: Instruction) -> Result<(), SimpletronError> {
        use crate::vm::operation::Opcode::*;

        match instr.opcode {
            Read => self.read(instr.operand, self.debug)?,
            Write => self.write(instr.operand, self.debug)?,
            WriteAcc => self.write_acc(self.debug)?,
            ReadI => self.read_i(self.debug)?,
            LoadM => self.load_m(instr.operand, self.debug)?,
            Store => self.store(instr.operand, self.debug)?,
            LoadI => self.load_i(instr.operand, self.debug)?,
            AddM => self.add_m(instr.operand, self.debug)?,
            SubM => self.sub_m(instr.operand, self.debug)?,
            DivM => self.div_m(instr.operand, self.debug)?,
            ModM => self.mod_m(instr.operand, self.debug)?,
            MulM => self.mul_m(instr.operand, self.debug)?,
            AddI => self.add_i(instr.operand, self.debug)?,
            SubI => self.sub_i(instr.operand, self.debug)?,
            DivI => self.div_i(instr.operand, self.debug)?,
            ModI => self.mod_i(instr.operand, self.debug)?,
            MulI => self.mul_i(instr.operand, self.debug)?,
            Jump => self.jump(instr.operand, self.debug)?,
            JumpIfNegative => self.jump_if_negative(instr.operand, self.debug)?,
            JumpIfZero => self.jump_if_zero(instr.operand, self.debug)?,
            Halt => self.halt(self.debug)?,
            JumpIfNotZero => self.jump_if_not_zero(instr.operand, self.debug)?,
            JumpIfGreaterThanZero => self.jump_if_greater_than_zero(instr.operand, self.debug)?,
        }
        Ok(())
    }

    fn debug(&self, debug: bool, msg: impl AsRef<str>) {
        if debug {
            println!("{}", "-".repeat(100));
            println!("[DEBUG] {}", msg.as_ref());
        }
    }
}

// instructions definitions
impl<P, M> Orchestrator<P, M>
where
    P: ProcessorInterface,
    M: MemoryInterface,
{
    fn read(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(
            debug,
            &format!("READ from keyboard -> Memory[+{:0>4}]", address),
        );

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
        let value = self.memory.read_data(address)?;
        self.debug(
            debug,
            &format!("WRITE <- Memory[+{:0>4}] = {}", address, value),
        );
        println!("Memory[{}] = {}", address, value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn write_acc(&mut self, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("Writing ACC value"));

        let value = self.cpu.get_acc_value();
        println!("ACC: {}", value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn read_i(&mut self, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("READ from keyboard -> ACC"));

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
        self.cpu.write_acc(value);
        self.cpu.increment_pc();
        Ok(())
    }

    fn load_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC <- Memory[+{:0>4}]", address));
        let value = self.memory.read_data(address)?;
        self.cpu.write_acc(value.parse().unwrap());
        self.cpu.increment_pc();
        Ok(())
    }

    fn store(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC -> Memory[+{:0>4}]", address));

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
        self.debug(debug, &format!("ACC <- {}", operand));

        self.cpu.write_acc(operand.try_into().unwrap());
        self.cpu.increment_pc();
        Ok(())
    }

    fn add_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC += value at Memory[+{:0>4}]", address));

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc + value);

        self.cpu.increment_pc();
        Ok(())
    }

    fn sub_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC -= value at Memory[+{:0>4}]", address));

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc - value);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mul_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC *= value at Memory[+{:0>4}]", address));

        let value: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc * value);

        self.cpu.increment_pc();
        Ok(())
    }

    fn div_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC /= value at Memory[+{:0>4}]", address));

        let divisor: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        if divisor == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc / divisor);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mod_m(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC %= value at Memory[+{:0>4}]", address));

        let divisor: i32 = self
            .memory
            .read_data(address)?
            .parse()
            .map_err(|_| SimpletronError::InvalidMemoryData(address))?;

        if divisor == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc % divisor) as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn add_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC += value {}", operand));

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc + operand as i32) as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn sub_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC -= value {}", operand));

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc((acc - operand as i32) as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mul_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC *= value {}", operand));

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc * operand as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn div_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC /= value {}", operand));

        if operand == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc / operand as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn mod_i(&mut self, operand: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("ACC %= value {}", operand));

        if operand == 0 {
            return Err(SimpletronError::DivisionByZero);
        }

        let acc = self.cpu.get_acc_value() as i32;
        self.cpu.write_acc(acc % operand as i32);

        self.cpu.increment_pc();
        Ok(())
    }

    fn jump(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("JUMP -> address Memory[+{:0>4}]", address));
        self.cpu.set_pc(address);
        Ok(())
    }

    fn jump_if_negative(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(
            debug,
            &format!("JUMP IF NEG -> address Memory[+{:0>4}]", address),
        );

        let acc = self.cpu.get_acc_value() as i32;
        if acc < 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }

    fn jump_if_zero(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(
            debug,
            &format!("JUMP IF ZERO -> address Memory[+{:0>4}]", address),
        );

        let acc = self.cpu.get_acc_value();
        if acc == 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }

    fn halt(&mut self, debug: bool) -> Result<(), SimpletronError> {
        self.debug(debug, &format!("HALT"));

        self.cpu.dump();
        self.memory.dump(-1);

        std::process::exit(0);
    }

    fn jump_if_not_zero(&mut self, address: usize, debug: bool) -> Result<(), SimpletronError> {
        self.debug(
            debug,
            &format!("JUMP IF NOT ZERO -> address Memory[+{:0>4}]", address),
        );

        let acc = self.cpu.get_acc_value();

        if acc != 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }

    fn jump_if_greater_than_zero(
        &mut self,
        address: usize,
        debug: bool,
    ) -> Result<(), SimpletronError> {
        self.debug(
            debug,
            &format!(
                "JUMP IF GREATER THAN ZERO -> address Memory[+{:0>4}]",
                address
            ),
        );

        let acc = self.cpu.get_acc_value();

        if acc > 0 {
            self.cpu.set_pc(address);
        } else {
            self.cpu.increment_pc();
        }
        Ok(())
    }
}
