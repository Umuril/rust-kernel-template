use core::arch::naked_asm;

use crate::{Context, kmain};
use embedded_io::Write;
mod uart;

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn multiboot() -> ! {
    naked_asm!(
        "
        .long 0x1BADB002
        .long 0x00000000
        .long 0xE4524FFE
    "
    );
}

#[unsafe(no_mangle)]
pub unsafe fn kstart() -> ! {
    let mut uart = uart::Uart::new();

    if uart.write_all(b"Bootstrapping kernel...\n").is_err() {
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
        HLT
        JMP halt
    "
    );
}
