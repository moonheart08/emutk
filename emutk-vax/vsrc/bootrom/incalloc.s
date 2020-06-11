.include "common.inc"

.text
.global _sbrk
.global _init_alloc
.global _ram_present
.global _ram_avail
.extern _print_str

.set ALLOC_CTRL_SPACE, 0x0800
.set RAM_SIZE, 8388608 # TODO: Probe this value from the hardware, don't hardcode it!

# IN: size (r0)
# CLOBBER: none
# STACK: 8 bytes
# OUT: ptr (r0)
# NOTES:
# The first allocation made by sbrk must be aligned on a 512 byte boundary.
# (Above can be controlled in _init_alloc)
_sbrk:
    pushl %r1 # save r1
    pushl %r2 # save r2
    moval (ALLOC_CTRL_SPACE+4), %r1
    movl (%r1), %r2
    addl2 %r0, (%r1) # Add requested size to control
    cmpl -4(%r1), (%r1) # Compare control with RAM end
    bgeq .oom
    movl %r2, %r0
    movl (%sp)+, %r2 # restore r2
    movl (%sp)+, %r1 # restore r1
    rsb
.oom:
    movl $sys_oom_message, %r0
    bsbw _print_str
    halt

# IN: none
# CLOBBER: none
# STACK: N/A
# OUT: none
_init_alloc:
    movl $RAM_SIZE, (ALLOC_CTRL_SPACE+0) # TODO: probe hardware for this instead.
    movl $0x1000, (ALLOC_CTRL_SPACE+4) 
    rsb

_ram_present:
    movl (ALLOC_CTRL_SPACE+0), %r0
    rsb

_ram_avail:
    subl3 (ALLOC_CTRL_SPACE+0), (ALLOC_CTRL_SPACE+4), %r0
    rsb

.section .rodata
sys_oom_message: 
.asciz "SYSOOM!\n"
