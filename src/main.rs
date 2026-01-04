use simpletron_rust::{
    SingleList, controller::Controller, memory::memory_loader::MemoryLoader,
    parser::lowlevel_parser::LowLevelParser, processor::simple_processor::SimpleProcessor,
};

fn main() {
    let debug = true;
    let mut memory = SingleList::new(None);
    let cpu = SimpleProcessor::new();

    {
        let parser = LowLevelParser::new(debug);
        let mut loader = MemoryLoader::new(parser, &mut memory, debug);
        loader.load("test.sml".to_string()).unwrap();
    }

    let mut controller = Controller::new(cpu, memory, debug);
    let _ = controller.run();
}
