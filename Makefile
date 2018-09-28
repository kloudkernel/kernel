all: src/main.rs
	bootimage build

boot: all
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-fros/debug/bootimage-fros.bin
