#![no_std]
#![no_main]
#![feature(panic_implementation)]

#[macro_use]
extern crate lazy_static;
extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    loop {}
}
