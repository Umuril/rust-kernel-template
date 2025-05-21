.SetupPaging:
    .code32

    // Disable old paging
    MOV eax, cr0
    AND eax, 0x7fffffff
    MOV cr0, eax

    // Enable Paging
    MOV edi, 0x200000
    MOV cr3, edi // &PML4T

    XOR eax, eax
    MOV ecx, 0x1000
    REP stosd // Repeated store (ecx times the eax value)

    // Set PML4T[0] -> PDPT[0]
    MOV edi, 0x200000
    MOV QWORD PTR [edi], 0x201003

    // Set PDPT[0] -> PDT[0]
    MOV edi, 0x201000
    MOV QWORD PTR [edi], 0x202003

    // Set PDT[0] -> PT[0]
    MOV edi, 0x202000
    MOV QWORD PTR [edi], 0x203003

    MOV edi, 0x203000
    MOV ebx, 0x00000003 // PT[i] -> Phisical address
    MOV ecx, 512 // Loop counter (loop instruction decrements it automatically)

    .SetEntryPT:
        MOV QWORD PTR [edi], ebx
        ADD ebx, 0x1000
        ADD edi, 8
        loop .SetEntryPT

    // Enable PAE-paging
    MOV eax, cr4
    OR eax, 1 << 5
    MOV cr4, eax

    RET

.EnablePaging:
    .code32

    MOV ecx, 0xC0000080
    RDMSR
    OR eax, 1 << 8
    WRMSR

    MOV eax, cr0
    OR eax, 1 << 31 | 1 << 0
    MOV cr0, eax

    RET