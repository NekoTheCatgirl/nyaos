#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

extern crate alloc;

mod applications;
mod fs;
mod init;
mod io;
mod memory;
mod task;
mod x86;

use bootloader::{entry_point, BootInfo};
use task::{executor::Executor, keyboard, Task};
use core::panic::PanicInfo;
use x86::hlt_loop;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL PANIC] {}", info);
    hlt_loop()
}

entry_point!(kernel_main);

/// This function handles bootup and setup for all the required functions. Including stdout and stdin
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to Nyaos! Preparing system hooks and interrupts! Please hold...");
    init::init(boot_info);

    let mut executor = Executor::new();
    executor.spawn(Task::new(kernel_async()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is used to enable async kernel mode. Moving the kernel on from here
async fn kernel_async() {
    println!("Async kernel activated");
    hlt_loop()
}