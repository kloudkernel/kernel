#![no_std]
#![feature(abi_x86_interrupt)]

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

extern crate ksys;
extern crate kloc;


#[macro_use]
pub mod vga_buffer;
#[macro_use]
pub mod serial;
pub mod interrupt;
pub mod gdt;

pub fn exit_qemu() {
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    unsafe {
        port.write(0);
    }
}
