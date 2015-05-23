MMCU=msp430g2553

.PHONY: all
all: main.elf

main.ll: main.rs
	rustc --emit llvm-ir -O -o $@ $^

main.s: main.ll
	llc -march=msp430 -o $@ $^


main.o: main.s
	msp430-elf-as -mmcu=$(MMCU) -o $@ $^

main.elf: main.o
	msp430-elf-gcc -mmcu=$(MMCU) -I$(MSP430_TI)/include -L$(MSP430_TI)/include -e main -o $@ $^

.PHONY: flash
flash:
	mspdebug rf2500 'prog main.elf'

.PHONY: clean
clean:
	-rm -f main.ll main.s main.o main.elf
