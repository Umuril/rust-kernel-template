use crate::{Context, kmain};
use core::arch::naked_asm;
use embedded_io::Write;
mod uart;

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

    if uart.write_all(b"Bootstrapping kernel...\n").is_err() {
        unsafe { halt() };
    }
    let _ = uart.flush();

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
