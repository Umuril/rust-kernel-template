ENTRY(multiboot)

MEMORY {
    RAM (rwx) : ORIGIN = 0x80200000, LENGTH = 4M
}

SECTIONS {
    . = 0x80200000;

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