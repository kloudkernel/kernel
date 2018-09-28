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
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
#[cfg(not(test))]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
#[cfg(not(feature = "integration-test"))]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    serial_println!("Hello world!");
    loop {}
}

#[no_mangle]
#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    serial_println!("Hello world!");
    exit_qemu();
    loop {}
}

pub fn exit_qemu() {
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    unsafe {
        port.write(0);
    }
}
