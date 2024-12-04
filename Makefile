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

QEMU_ARGS := -d int,guest_errors \
	        -D /tmp/qemu.log \
			-machine virt \
			-m 4G \
			-nographic \
			-bios $(BOOTLOADER) \
			-device loader,file=$(HYPERVISOR_BIN),addr=$(HYPERVISOR_ENTRY_PA) \
			-drive file=guests/rCore-Tutorial-v3/fs.img,if=none,format=raw,id=x0 \
			-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

run: $(HYPERVISOR_BIN)
	@qemu-system-riscv64 $(QEMU_ARGS)

gdbserver: $(HYPERVISOR_BIN)
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

clean:
	@cargo clean