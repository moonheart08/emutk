# We are at 0x7000_0000 in memory, as part of ROM.
.section .entry

.global _start
.extern _print_str

_start:
.org 0x00
processor_restart_address: brw _true_start 
.org 0x04
sys_type: .4byte 0x00000000
.org 0x08
version: .4byte 0x01
.org 0x0C
index: .4byte 0x00
.org 0x10
manufacture_check:
.4byte 0x55555555, 0xAAAAAAAA, 0x33333333
.org 0x1C
rom_len: .4byte 0x00010000
.org 0x20
interrupt_vectors:
#TODO

.org 0x88
_true_start:
    
_getc:

_putc:
.section .rodata
hello_world: 
.asciz "KA420 Booting\n"
