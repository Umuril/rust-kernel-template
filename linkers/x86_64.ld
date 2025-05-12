ENTRY(kstart)

MEMORY {
    RAM (rwx) : ORIGIN = 0x100000, LENGTH = 4M
}

SECTIONS {
    . = 0x100000;

    .boot2 : {
        KEEP(*(.boot2))
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