SECTIONS
{
    .rodata : {
        ASSERT(. > 0xffffffff00000000, "Expected higher half address")

        KEEP(*(.rodata.symbol_desc*))
        . = ALIGN(8);
        *(.rodata*)
    }
}