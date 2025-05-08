ENTRY(kstart)

SECTIONS {

    .boot2 : {
        __mb_start = .;
            *(.multiboot)
        __mb_end = .;
        /* ASSERT(__mb_start != __mb_end, "Empty .multiboot section"); */

        *(.boot2)
    }

    .text : { *(.text .text.*) }

    /DISCARD/ : {
        *(.dynsym) *(.gnu.hash) *(.hash) *(.dynstr) *(dynamic)
    }
}