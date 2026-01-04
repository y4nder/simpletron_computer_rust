use std::{fs::File, io::Read};

use simpletron_rust::{
    assembler::{self},
    vm::error::SimpletronError,
};

fn main() -> Result<(), SimpletronError> {
    let path = "mnemonic.m";

    let mut source = String::new();
    File::open(path)?.read_to_string(&mut source)?;

    let program = assembler::assemble(&source)?;
    println!("{:?}", program);

    Ok(())
}

//fn main() {
//    let args = CliArgs::parse();
//    if let Err(e) = run(args) {
//        eprintln!("error: {}", e);
//        std::process::exit(1);
//    }
//}
//
//fn run(args: CliArgs) -> Result<(), SimpletronError> {
//    let mut memory = SimpleMemory::new(None);
//    let cpu = SimpleProcessor::new();
//    {
//        let mut loader =
//            MemoryLoader::new(LowLevelParser::new(args.debug), &mut memory, args.debug);
//        loader.load(args.filename)?;
//    }
//    let mut controller = Orchestrator::new(cpu, memory, args.debug);
//    controller.run()?;
//    Ok(())
//}
