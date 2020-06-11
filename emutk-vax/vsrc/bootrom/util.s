.include "common.inc"

.text
.set print_buffer, 0x280 # 256 bytes after the head of stack
.global _print_str
.global _print_dec
.global _query

__real_print_str:
.print_loop:
    mtpr %r1, $TXDB
_print_str:
    movb (%r0)+, %r1
    bneq .print_loop
    rsb

# IN: value (r0)
# SIDE-EFFECTS: Prints r0 to serial as hex
# NOTES:
# mildly inefficient? :|
_print_dec:
    clrl %r1
    # r1 is used, r0:r1 is a pair for ediv
    movl $10, %r2
    movl $decimal_table, %r3
    movl $print_buffer, %r5
.dec_cvt_loop:
    ediv %r2, %r0, %r0, %r4
    movb (%r3)[%r4], (%r5)+
    tstl %r0
    bneq .dec_cvt_loop
.dec_prt_loop:
    movb -(%r5), %r0
    beql .print_dec_done
    mtpr %r0, $TXDB
    brb .dec_prt_loop
.print_dec_done:
    rsb

_getc:


# IN: *buf (r0)
_query:
    pushl %r0 # save ptr to input buffer
    movl $query_indic, %r0
    bsbw _print_str
_query_loop:
    brb _query_loop

.section .rodata
decimal_table:
.ascii "0123456789"
query_indic:
.asciz "> "