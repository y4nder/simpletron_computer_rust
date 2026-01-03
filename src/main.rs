use simpletron_rust::{
    SingleList,
    memory::{memory_interface::MemoryInterface, memory_payload::MemoryPayload},
};

fn main() {
    let mut memory = SingleList::new(None);

    let payload = MemoryPayload {
        address: 10,
        data: 1094.into(),
    };

    match memory.store_data(payload) {
        Ok(_) => {}
        Err(_) => {}
    };

    memory.dump(10);
}
