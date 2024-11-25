# riscv-hypervisor
- [ ] CPU virtualization
- [ ] Memory virtualization (two-stage address translation)
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
```