.include "common.inc"

.text
.global _print_str
_print_str:
    brb .entry
.loop:
    mtpr %r1, $TXDB
.entry:
    movb (%r0)+, %r1
    cmpb %r1, $0
    bneq .loop
