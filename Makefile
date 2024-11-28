HYPERVISOR_ELF := target/riscv64gc-unknown-none-elf/release/riscv-hypervisor
HYPERVISOR_BIN := $(HYPERVISOR_ELF).bin
BOOTLOADER := bootloader/rustsbi-qemu-2024-03-24.bin
HYPERVISOR_ENTRY_PA := 0x80200000

LOG ?= INFO

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf:
	LOG=$(LOG) cargo build --release --target riscv64gc-unknown-none-elf

$(HYPERVISOR_BIN): elf
	@$(OBJCOPY) $(HYPERVISOR_ELF) --strip-all -O binary $@

qemu: $(HYPERVISOR_BIN)
	@qemu-system-riscv64 \
			-machine virt \
			-m 1G \
			-nographic \
			-bios $(BOOTLOADER) \
			-device loader,file=$(HYPERVISOR_BIN),addr=$(HYPERVISOR_ENTRY_PA) \
			-drive file=guests/rCore-Tutorial-v3/fs.img,if=none,format=raw,id=x0 \
			-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

clean:
	@cargo clean