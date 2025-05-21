use core::arch::{global_asm, naked_asm};

use crate::{Context, kmain};
mod paging;
mod uart;
use core::fmt::Write;

global_asm!(include_str!("boot2.S"));

#[unsafe(link_section = ".multiboot_header")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn multiboot_header() -> ! {
    naked_asm!(
        "
        .long 0x1BADB002
        .long 0x00000000
        .long 0xE4524FFE
    "
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe fn bootloader() {
    naked_asm!(
        "
        MOV rsp, 0x200000 
        CALL .SetupPaging
        CALL .EnablePaging
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
        HLT
        JMP halt
    "
    );
}
