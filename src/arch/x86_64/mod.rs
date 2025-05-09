use core::arch::naked_asm;

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
#[unsafe(naked)]
pub unsafe fn kstart() -> ! {
    naked_asm!(
        "
        JMP kmain
    "
    );
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
