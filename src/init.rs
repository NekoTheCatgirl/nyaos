use alloc::{boxed::Box, vec};
use bootloader::BootInfo;
use x86_64::VirtAddr;

use crate::{
    memory::{self, allocator, BootInfoFrameAllocator},
    println, x86,
};

pub fn init(boot_info: &'static BootInfo) {
    println!("Initializing x86");
    x86::init();
    println!("Preparing allocators");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    println!("Initializing Heap");
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Test single allocation and deallocation
    let x = Box::new(41);
    println!("Heap allocation test passed");
    drop(x);
    println!("Heap deallocation test passed");
    
    // Test multiple allocations and deallocations
    let mut allocations = vec![];
    for i in 0..10 {
        let y = Box::new(i);
        allocations.push(y);
        println!("Allocated Box with value: {}", i);
    }
    println!("Multiple heap allocation test passed");

    // Drop all allocations
    for y in allocations {
        drop(y);
    }
    println!("Multiple heap deallocation test passed");
}
