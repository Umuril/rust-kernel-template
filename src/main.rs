#![no_std]
#![no_main]
#![feature(naked_functions_rustic_abi)]

mod arch;

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    ".section .multiboot",
    ".align 4",
    ".long 0x1BADB002",          // magic number
    ".long 0x0",                 // flags
    ".long -(0x1BADB002 + 0x0)", // checksum
);

#[unsafe(no_mangle)]
pub fn kmain() -> ! {
    loop {}
}
