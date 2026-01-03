use simpletron_rust::{
    SingleList,
    memory::{memory_interface::MemoryInterface, memory_loader::MemoryLoader},
    parser::lowlevel_parser::LowLevelParser,
};

fn main() {
    let debug = true;
    let mut memory = SingleList::new(None);

    {
        let parser = LowLevelParser::new(debug);
        let mut loader = MemoryLoader::new(parser, &mut memory, debug);
        loader.load("test.sml".to_string()).unwrap();
    }

    memory.dump(-1);
}
