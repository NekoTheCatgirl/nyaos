#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

mod allocators;
mod sh;
mod mem;

extern crate alloc;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {
        
    }
}