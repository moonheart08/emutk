.include "common.inc"

.text
.global _print_str
.global _putc

_print_str:
    brb .entry
.loop:
    mtpr %r1, $TXDB
.entry:
    movb (%r0)+, %r1
    bneq .loop
    rsb


_putc: 
    mtpr %R2, $TXDB
    rsb