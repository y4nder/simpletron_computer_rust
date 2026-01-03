use simpletron_rust::{
    SingleList,
    memory::{memory_interface::MemoryInterface, memory_loader::MemoryLoader},
    parser::lowlevel_parser::LowLevelParser,
};

fn main() {
    let debug = true;
    let parser = LowLevelParser::new(debug);
    let memory = SingleList::new(None);

    let mut loader = MemoryLoader::new(parser, memory, debug);
    loader.load("test.sml".to_string()).unwrap();

    loader.memory().dump(-1);
}
