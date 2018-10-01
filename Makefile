all: src/main.rs
	bootimage build

boot: all
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-kloudkernel/debug/bootimage-kloudkernel.bin -serial mon:stdio \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-display none
boot-display: all
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-kloudkernel/debug/bootimage-kloudkernel.bin -serial mon:stdio \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 
test:
	cargo test
	bootimage test
