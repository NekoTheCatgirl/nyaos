#![no_std]
#![no_main]

mod io;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    loop {}
}