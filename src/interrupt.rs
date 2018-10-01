use gdt;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("BREAKPOINT: {:#?}", stack_frame);
    serial_println!("BREAKPOINT: {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    println!(
        "DOUBLE FAULT: {:#?}\n Error Code: {}",
        stack_frame, error_code
    );
    serial_println!(
        "DOUBLE FAULT: {:#?}\n Error Code: {}",
        stack_frame,
        error_code
    );
    loop {}
}
