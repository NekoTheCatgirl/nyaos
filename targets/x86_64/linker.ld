ENTRY(start)

SECTIONS
{
    . = 1M;
    .boot : 
    {
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text._start)
        *(.text*)
    }

    .eh_frame :
    {
        *(.eh_frame*)
    }

    .note : 
    {
        *(.note*)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .data :
    {
        *(.data*)
    }

    .bss :
    {
        *(.bss*)
        *(COMMON)
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}