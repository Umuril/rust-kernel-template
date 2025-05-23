ENTRY(multiboot)

RAM_START         = 0x80000000;
FIRMWARE_SIZE     =   0x200000;
MAX_KERNEL_SIZE   =   0xA00000; /* 10 MiB */
RAM_SIZE          =  0x8000000; /* 128 MiB */

KERNEL_START_ADDR = RAM_START + FIRMWARE_SIZE;
RAM_END = RAM_START + RAM_SIZE;


MEMORY {
    FIRMWARE (r) : ORIGIN = RAM_START, LENGTH = FIRMWARE_SIZE
    RAM (rwx)    : ORIGIN = RAM_START + FIRMWARE_SIZE, LENGTH = RAM_SIZE - FIRMWARE_SIZE
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

    ASSERT(. < KERNEL_START_ADDR + MAX_KERNEL_SIZE, "Error: size of kernel overflows into kernel heap")
    
    . = KERNEL_START_ADDR + MAX_KERNEL_SIZE;

    /DISCARD/ : {
        *(.dynsym) *(.gnu.hash) *(.hash) *(.dynstr) *(.dynamic) *(.comment)
    }
}