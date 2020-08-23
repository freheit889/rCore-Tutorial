target=riscv64gc-unknown-none-elf
mode=debug
kernel = ../target/$(target)/$(mode)/sdtest
image = ../target/$(target)/$(mode)/kernel.bin
k210-serialport=/dev/ttyUSB0

export USER_IMG =sdtest/raw.img


kernel:
	@cp -rf ../../../user/build/raw.img .
	@cargo build

image: kernel
	@rust-objcopy $(kernel) --strip-all -O binary $(image)

down:image
	@python3 built/kflash.py -p $(k210-serialport) -b 1500000 -t $(image)
	@rm raw.img
clean:
	@cargo clean
