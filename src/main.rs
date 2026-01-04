use clap::Parser;
use simpletron_rust::{
    SimpleMemory, cli::CliArgs, controller::Controller, error::SimpletronError,
    memory::memory_loader::MemoryLoader, parser::lowlevel_parser::LowLevelParser,
    processor::simple_processor::SimpleProcessor,
};

fn main() {
    let args = CliArgs::parse();
    if let Err(e) = run(args) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: CliArgs) -> Result<(), SimpletronError> {
    let mut memory = SimpleMemory::new(None);
    let cpu = SimpleProcessor::new();
    {
        let mut loader =
            MemoryLoader::new(LowLevelParser::new(args.debug), &mut memory, args.debug);
        loader.load(args.filename)?;
    }
    let mut controller = Controller::new(cpu, memory, args.debug);
    controller.run()?;
    Ok(())
}
