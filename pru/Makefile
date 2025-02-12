PRU_SWPKG = /usr/local/src/ti_clpru

CC = $(PRU_SWPKG)/bin/clpru
LD = $(PRU_SWPKG)/bin/lnkpru
HEX = $(PRU_SWPKG)/bin/hexpru

LINKER_SCRIPT = AM3359_PRU.cmd
CFLAGS = --include_path=$(PRU_SWPKG)/include \
         --silicon_version=3 \
		 --hardware_mac=on \
		 --opt_level=2 \
		 --asm_listing \
		 --issue_remarks \
		 --verbose_diagnostics \
		 --emit_warnings_as_errors

# Add library path
LDFLAGS = --ram_model \
          -I=$(PRU_SWPKG)/lib \
		  -i=$(PRU_SWPKG)/lib/libc.a \
		  --verbose_diagnostics \
		  -w \
		  --issue_remarks \
	

.PHONY: all
all: mac.bin fib.bin

%.bin: %.elf
	$(HEX) -b -o $@ $< 


%.elf: %.obj $(LINKER_SCRIPT)
	$(CC) -z $(LINKER_SCRIPT) $(LDFLAGS) -o $@ $<

%.obj: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f *.bin *.elf *.obj *.lst *.b10
