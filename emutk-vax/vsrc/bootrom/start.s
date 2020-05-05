# We are at 0x7000_0000 in memory, as part of ROM.
.section .entry

.global _start
.extern _print_str

_start:
    movl $hello_world, %r0
    bsbb _print_str
    halt

.section .rodata
hello_world: 
.asciz "Hello, World!\n"
