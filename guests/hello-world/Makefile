TARGET := hello-world
TARGET_ELF := target/riscv64gc-unknown-none-elf/release/$(TARGET)
TARGET_BIN := $(TARGET).bin

all: clean $(TARGET_BIN) FORCE

$(TARGET_BIN): $(TARGET_ELF)
	@rust-objcopy --binary-architecture=riscv64 --strip-all -O binary $< $@

$(TARGET_ELF):
	@cargo build -p $(TARGET) --target riscv64gc-unknown-none-elf --release

clean:
	@rm -rf ./$(TARGET_BIN)
	@cargo clean

FORCE:

.PHONY: FORCE