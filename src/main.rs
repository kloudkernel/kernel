#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(panic_implementation)]

#[macro_use]
extern crate kloudkernel;
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

