#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(panic_implementation)]

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate array_init;

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
#[cfg(not(test))]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    loop {}
}
