This folder contains various source files used by the emulator, and a little bit of docs for them.

## Building:
You will need to have a cross-compilation ready version of binutils for vax-none-netbsdelf installed to build these files.

## common.asm
What it says on the tin. Contains common routines.

## genboot.asm
Generic Bootloader. Provides a VAX-like boot prompt, and handles testing the system to make sure it's functional. Should be modified with care, it's designed to mimic the real-world VAX boot prompt.