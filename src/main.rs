#![no_std]
#![no_main]
#![feature(naked_functions_rustic_abi)]

mod arch;

use core::panic::PanicInfo;

use arch::x86_64::halt;

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { halt() };
}

pub struct Context<T: embedded_io::Write> {
    pub primary_log: T,
}

pub fn kmain<T: embedded_io::Write>(mut ctx: Context<T>) -> ! {

    if ctx.primary_log.write_all(b"Hello, World!\n").is_err() {
        unsafe { halt() };
    };

    unsafe { halt() };
}
