#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
extern crate x86;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate core;

#[macro_use]
mod console;
mod interrupts;
pub mod banner;
pub mod gdt;
pub mod lapic;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt();
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    banner::boot_banner();

    gdt::init();
    interrupts::init_idt();

    interrupts::init_irqs();
    println!("Enabling interrupts");
    x86_64::instructions::interrupts::enable();
    println!("Enabled");

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); 
     
    println!("boot ok");

    halt();
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
