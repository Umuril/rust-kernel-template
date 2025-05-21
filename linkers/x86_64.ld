ENTRY(bootloader)

MEMORY {
    RAM (rwx) : ORIGIN = 0x100000, LENGTH = 4M
}

SECTIONS {
    . = 0x100000;

    .multiboot_header : {
        KEEP(*(.multiboot_header))
    } > RAM

    .text : {
        *(.text .text.*)
    } > RAM
    
    .rodata : {
        *(.rodata .rodata.*)
    } > RAM

    .data : {
        *(.data .data.*)
    } > RAM

    .bss : {
        *(.bss .bss.*)
    } > RAM

    /DISCARD/ : {
        *(.dynsym) *(.gnu.hash) *(.hash) *(.dynstr) *(.dynamic) *(.comment)
    }
}