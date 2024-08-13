#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("boot.asm"));

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 4*1024*1024] = [0; 4*1024*1024];

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}

#[no_mangle]
fn main() -> ! {
    loop {

    }
}
