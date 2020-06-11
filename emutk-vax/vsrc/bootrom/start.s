.include "common.inc"

.section .entry

.global _start
.extern _print_str
.extern _init_alloc
.extern _sbrk
.extern _query

_start:
    movl $0x200, %sp # begin stack
    bsbw _init_alloc
    movl $hello_world, %r0
    #movl $0, %r1
    #movc3 $15, (%r0), (%r1)
    #movl $0, %r0
    bsbw _print_str
    # Allocate system control blocks
    movw $0x200, %r1
    movl %r1, %r0
    bsbw _sbrk # Allocate BootROM Management Block
    movl %r0, %r2
    movl %r1, %r0
    bsbw _sbrk # Allocate System Control Block
    mtpr %r0, $SCBB
    
    # Print RAM messages.
    bsbw _ram_present
    divl3 %r0, $1024, %r0
    bsbw _print_dec
    movl $ram_message_1, %r0
    bsbw _print_str
    bsbw _ram_avail
    divl3 %r0, $1024, %r0
    bsbw _print_dec
    movl $ram_message_2, %r0
    bsbw _print_str

    bsbw _query
    halt

.section .rodata
hello_world: 
.asciz "reFORTH v0.0.0\n"

ram_message_1:
.asciz "K of RAM present; "

ram_message_2:
.asciz "K of RAM available\n"
