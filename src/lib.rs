#![no_std]
#![allow(internal_features)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(lang_items)]
#![feature(ptr_internals)]

extern crate alloc;

mod allocator;
mod mem;
mod sh;
#[macro_use]
#[allow(dead_code)]
mod framebuffers;
#[cfg(feature = "testing-kernel")]
mod test;
mod net;

use core::panic::PanicInfo;

use multiboot2::BootInformationHeader;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[PANIC] Kernel panic in {} at line {}:{}!\n{}",
            location.file(),
            location.line(),
            location.column(),
            info.message()
        );
    } else {
        println!("[PANIC] Kernel panic without location!\n{}", info.message());
    }
    loop {}
}

/// This function handles executing all the required tests for the kernel, to enable them use the feature flag "testing-kernel"
#[cfg(feature = "testing-kernel")]
fn tests() {
    println!("[WARN] Testing kernel is active, debug features are exposed and available to all");
    println!("[WARN] Tests will commence now!");
    test::exec_tests();
    println!("[OK] Tests complete");
}

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_info: u32) -> ! {
    framebuffers::clear_screen();
    println!("[OK] Rust kernel booting.");

    let boot_info = unsafe { 
        multiboot2::BootInformation::load(multiboot_info as *const BootInformationHeader).unwrap() 
    };

    allocator::init_heap(&boot_info);

    #[cfg(feature = "testing-kernel")]
    tests();

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}


fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}