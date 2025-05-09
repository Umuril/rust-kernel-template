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

#[unsafe(no_mangle)]
pub fn kmain() -> ! {
    const VIDEO_MEMORY_WIDTH: isize = 80;
    const _VIDEO_MEMORY_HEIGHT: isize = 25;
    const ROW: isize = 12;

    let ptr = (0xb8000) as *mut u16;

    for (i, &c) in "Hello, world!".as_bytes().iter().enumerate() {
        unsafe {
            let dst = ptr.offset(ROW * VIDEO_MEMORY_WIDTH).offset((80 - 13) / 2 ).add(i);
            core::ptr::write_volatile(dst, 0x0F00 + c as u16);
        }
    }

    unsafe { halt() };
}
