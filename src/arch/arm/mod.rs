use crate::{Context, kmain};
use core::arch::naked_asm;
mod uart;
use core::fmt::Write;

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn multiboot() -> ! {
    naked_asm!(
        "
        LDR sp, =0x44000000
        BL kstart
    "
    );
}

#[unsafe(no_mangle)]
pub unsafe fn kstart() -> ! {
    let mut uart = uart::Uart::new();

    if uart.write_str("Bootstrapping kernel...\n").is_err() {
        unsafe { halt() };
    }

    let ctx = Context { primary_log: uart };

    kmain(ctx);
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn halt() -> ! {
    naked_asm!(
        "
        WFI
        BL halt
    "
    );
}
