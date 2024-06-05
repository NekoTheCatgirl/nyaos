#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}

/// This function handles bootup and setup for all the required functions. Including stdout and stdin
fn kernel_main() -> ! {
    loop {
        
    }
}