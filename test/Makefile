TEXT=0x0000

kernel.bin: main
	objbin-- ${TEXT} kernel.bin main | sort > kernel.map

main: crt0.o main.o
	ld-- main crt0.o main.o > main.sym

main.o: main.cmm crt0.hmm
	cm2e -K -c main.cmm

crt0.o: crt0.s
	as-- crt0.s

clean:
	rm -f main main.s *.lst *.bin *.o *.hex *.sym *.map *~
