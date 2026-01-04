use std::{fs::File, io::Read};

use clap::Parser;
use simpletron_rust::{
    assembler,
    cli::CliArgs,
    orchestrator::Orchestrator,
    vm::{
        error::SimpletronError,
        memory::{MemoryLoader, SimpleMemory},
        processor::SimpleProcessor,
    },
};

fn main() {
    let args = CliArgs::parse();

    if let Err(err) = run(args) {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
    // let path = "mnemonic.m";

    // let mut source = String::new();
    // File::open(path)?.read_to_string(&mut source)?;

    // let program = assembler::assemble(&source)?;

    // let mut memory = SimpleMemory::new(None);
    // {
    //     let mut loader = MemoryLoader::new(&mut memory, true);
    //     loader.load_program(&program)?;
    // }

    // let cpu = SimpleProcessor::new();
    // let mut controller = Orchestrator::new(cpu, memory, true);
    // controller.run()?;

    // Ok(())
}

fn run(args: CliArgs) -> Result<(), SimpletronError> {
    let path = args.filename;

    let mut source = String::new();
    File::open(path)?.read_to_string(&mut source)?;

    let program = assembler::assemble(&source)?;

    let mut memory = SimpleMemory::new(None);
    {
        let mut loader = MemoryLoader::new(&mut memory, args.debug);
        loader.load_program(&program)?;
        println!("{:?}", program)
    }

    let cpu = SimpleProcessor::new();
    let mut controller = Orchestrator::new(cpu, memory, args.debug);
    controller.run()?;

    Ok(())
}

//use std::{fs::File, io::Read};
//
//use simpletron_rust::{
//    assembler::{self},
//    vm::error::SimpletronError,
//};
//
//fn main() -> Result<(), SimpletronError> {
//    let path = "mnemonic.m";
//
//    let mut source = String::new();
//    File::open(path)?.read_to_string(&mut source)?;
//
//    let program = assembler::assemble(&source)?;
//    println!("{:?}", program);
//
//    Ok(())
//}

//fn main() {
//    let args = CliArgs::parse();
//    if let Err(e) = run(args) {
//        eprintln!("error: {}", e);
//        std::process::exit(1);
//    }
//}
//
//fn run(args: cliargs) -> result<(), simpletronerror> {
//    let mut memory = simplememory::new(none);
//    let cpu = simpleprocessor::new();
//    {
//        let mut loader =
//            memoryloader::new(lowlevelparser::new(args.debug), &mut memory, args.debug);
//        loader.load(args.filename)?;
//    }
//    let mut controller = orchestrator::new(cpu, memory, args.debug);
//    controller.run()?;
//    ok(())
//}
