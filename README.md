# riscv-hypervisor
- [ ] CPU virtualization
- [x] Memory virtualization (two-stage address translation)
- [x] Handle sbi calls
- [x] Parsing device tree
- [ ] Multi-core support
- [ ] Multi-guest support
- [ ] IOMMU enabled

## Get started
1.Install target
```bash
rustup target add riscv64gc-unknown-none-elf
```
2.Install cargo-binutils
```bash
cargo install cargo-binutils
```
3.Install qemu
```bash
brew install qemu
apt install qemu-system-riscv64
```