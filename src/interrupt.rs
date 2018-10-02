use gdt;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable, PageFaultErrorCode};

macro_rules! handler_with_error_code {
    ($name: ident, $msg: expr) => {
        extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
            let msg = $msg;
            println!("{}: {:#?}\n Error Code: {}", msg, stack_frame, error_code);
            serial_println!("{}: {:#?}\n Error Code: {}", msg, stack_frame, error_code);

            loop {}
        }
    };
}

macro_rules! handler {
    ($name: ident, $msg: expr) => {
        extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame) {
            let msg = $msg;
            println!("{}: {:#?}\n", msg, stack_frame);
            serial_println!("{}: {:#?}\n", msg, stack_frame);

            loop {}
        }
    };
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt.general_protection_fault.set_handler_fn(general_protection_handler);
        idt.alignment_check.set_handler_fn(alingment_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}
handler_with_error_code!(alingment_handler, "ALIGNEMNT FAULT");
handler_with_error_code!(double_fault_handler, "DOUBLE FAULT");
handler_with_error_code!(general_protection_handler, "GENERAL PROTECTION FAULT");

handler!(breakpoint_handler, "BREAKPOINT");

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!(
        "PAGE FAULT: {:#?}\n Error Code: {:#?}",
        stack_frame, error_code
    );
    serial_println!(
        "PAGE_FAULT: {:#?}\n Error Code: {:#?}",
        stack_frame,
        error_code
    );

    loop {}
}
