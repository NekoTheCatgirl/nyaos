//! This module contains the required spec for handling the x86_64 interrupts and gdtmod interrupts;
mod gdt;
mod interrupts;

/// Init all the required things within x86_64
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; 
    x86_64::instructions::interrupts::enable(); 
}

/// Halt the cpu forever
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}