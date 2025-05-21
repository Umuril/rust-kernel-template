use crate::{Context, kmain};
use core::arch::naked_asm;
use core::fmt::Write;
mod uart;

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn multiboot() -> ! {
    naked_asm!(
        "
        LI sp, 0x88000000
        CALL kstart
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
        CALL halt
    "
    );
}
