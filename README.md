# Rust Kernel Template

### Implementing new architecture

```bash
rustc --print target-list
rustc +nightly -Z unstable-options --target=riscv64gc-unknown-none-elf --print target-spec-json > targets/riscv64.json
```

### Commands to try to solve problems

```bash
sudo apt install grub-pc-bin
cargo +nightly build -Z build-std=core --target riscv64gc-unknown-none-elf
rustup default nightly
rustup install nightly
rustup component add rust-src --toolchain nightly
rustup target add riscv64gc-unknown-none-elf --toolchain nightly
```

### DTS example

```bash
qemu-system-riscv64 -M virt,dumpdtb=target/virt.dtb
dtc -I dtb -O dts -o target/virt.dts target/virt.dtb
```