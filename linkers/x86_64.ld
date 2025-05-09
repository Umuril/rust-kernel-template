ENTRY(kstart)

SECTIONS {
    . = 0x100000;

    .boot2 : {
        KEEP(*(.boot2))
    }

    .text : { *(.text .text.*) }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    /DISCARD/ : {
        *(.dynsym) *(.gnu.hash) *(.hash) *(.dynstr) *(.dynamic) *(.comment)
    }
}