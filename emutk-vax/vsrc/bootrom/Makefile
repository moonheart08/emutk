AS := vax-unknown-netbsdelf-as
LD := vax-unknown-netbsdelf-ld
OBJCOPY := vax-unknown-netbsdelf-objcopy

S_FILES := $(wildcard *.s)
OBJS := start.o util.o dict.o forth.o corewords.o incalloc.o

%.o : %.s
	$(AS) $< -o $@

bootloader.elf : $(OBJS)
	$(LD) -o $@ -T link.ld $(OBJS)

bootloader.bin: bootloader.elf
	$(OBJCOPY) --strip-all --set-start 0 -O binary bootloader.elf bootloader.bin

clean:
	rm $(OBJS) bootloader.elf bootloader.bin

.PHONY: clean