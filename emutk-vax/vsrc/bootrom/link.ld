MEMORY {
    /* Minimum RAM in a machine is 1MiB, but BIOS should minimize it's use of system memory. */
    sysram (w!x) : ORIGIN = 0x00000000, LENGTH = 4K

    rom (rx) : ORIGIN = 0x10000000, LENGTH = 256K
}

SECTIONS
{
    .entry : {
        *(.entry .entry.*);
        . += SIZEOF(.entry);
    } >rom = 0x00


    .text : {
        *(.text .text.*);
        . += SIZEOF(.text);
    } >rom = 0x00

    .rodata : {
        *(.rodata .rodata.*);
    } >rom = 0x00

    .data : { 
        __data_start = ABSOLUTE(.);
        *(.data .data.*);
        __data_end = ABSOLUTE(.);
    } >sysram AT>rom

    .bss : {
        *(.bss .bss.*);
        __bss_end = ABSOLUTE(.);
    } >sysram

    .stab            0 : { *(.stab) }
    .stabstr         0 : { *(.stabstr) }
    .stab.excl       0 : { *(.stab.excl) }
    .stab.exclstr    0 : { *(.stab.exclstr) }
    .stab.index      0 : { *(.stab.index) }
    .stab.indexstr   0 : { *(.stab.indexstr) }
    .comment         0 : { *(.comment) }
    .debug           0 : { *(.debug) }
    .line            0 : { *(.line) }
    .debug_srcinfo   0 : { *(.debug_srcinfo) }
    .debug_sfnames   0 : { *(.debug_sfnames) }
    .debug_aranges   0 : { *(.debug_aranges) }
    .debug_pubnames  0 : { *(.debug_pubnames) }
    .debug_info      0 : { *(.debug_info) }
    .debug_abbrev    0 : { *(.debug_abbrev) }
    .debug_line      0 : { *(.debug_line) }
   	.debug_frame     0 : { *(.debug_frame) }
    .debug_str       0 : { *(.debug_str) }
    .debug_loc       0 : { *(.debug_loc) }
    .debug_macinfo   0 : { *(.debug_macinfo) }

    /DISCARD/ : { *(*) }
}