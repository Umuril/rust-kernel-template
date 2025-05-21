#![no_std]
#![no_main]
#![feature(naked_functions_rustic_abi)]

mod arch;
use arch::*;

use core::panic::PanicInfo;

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { halt() };
}

pub struct Context<T: core::fmt::Write> {
    pub primary_log: T,
}

pub fn kmain<T: core::fmt::Write>(mut ctx: Context<T>) -> ! {
    if ctx.primary_log.write_str("Hello, World!\n").is_err() {
        unsafe { halt() };
    };

    unsafe { halt() };
}
