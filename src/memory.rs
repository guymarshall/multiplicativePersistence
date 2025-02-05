use sysinfo::System;

pub(crate) fn calculate_chunk_size() -> usize {
    let mut system: System = System::new_all();
    system.refresh_memory();

    let total_memory_bytes: u64 = system.total_memory();
    let memory_budget: usize = (total_memory_bytes / 5) as usize;

    let element_size: usize = size_of::<usize>();

    let vec_capacity: usize = memory_budget / element_size;

    vec_capacity
}
