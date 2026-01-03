use simpletron_rust::{
    SingleList,
    memory::{memory_interface::MemoryInterface, memory_payload::MemoryPayload},
};

fn main() {
    let mut memory = SingleList::new(None);

    let payload = MemoryPayload {
        address: 99,
        data: 1094.into(),
    };

    match memory.store_data(payload) {
        Ok(_) => println!("value inserted"),
        Err(e) => eprintln!("error: {}", e),
    };

    let address = 100;

    match memory.read_data(address) {
        Ok(val) => println!("value {} is at address: {}", val, address),
        Err(e) => eprintln!("error: {}", e),
    }

    memory.dump(10);
}
