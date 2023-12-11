.section .text.boot
// .global kernelMain

_start:
    // Setting up stack to safe address
    mov x0, #0x80000
    mov sp, x0

    // Disabling interrupts
    msr daifset, 0xf

    // Identity map all memory
    mov x0, #0x80000
    ldr x1, =(0b00 << 0 | 0b00 << 2 | 0b0 << 4 | 0b0 << 5 | 0b0 << 6 | 0b1 << 7)
    ldr x2, =(0b00 << 0 | 0b11 << 8 | 0b100 << 29)
    ldr x3, =(0b1 << 0 | 0b1 << 1)
    mov x10, #0x0000FFFFFFFFFFFF
    msr ttbr0_el1, x10
    isb

    // Jump to Rust main
    bl kernelMain

.section .text
// .global kernelMain
// kernelMain:
//    b _start