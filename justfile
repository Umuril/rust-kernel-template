set dotenv-load

TARGET_DIR := if env("PROFILE") == "dev" {
  "debug"
} else {
  env("PROFILE")
}

default:
    @just --list --justfile {{justfile()}}

build:
    cargo clean -p rust-kernel-template # Force rebuild to get changes to linker scripts. Move to build.rs in the future
    cargo rustc --profile=$PROFILE --target=targets/$ARCH.json -- -Clink-arg=-T -Clink-arg=linkers/$ARCH.ld

readelf: build
    $BINUTILS-readelf -a target/$ARCH/{{TARGET_DIR}}/rust-kernel-template

qemu:
    rm -fr ./isodir
    mkdir -p isodir/boot/grub
    objcopy -O binary target/$ARCH/{{TARGET_DIR}}/rust-kernel-template isodir/boot/kernel.bin
    echo 'multiboot /boot/kernel.bin\nboot' > isodir/boot/grub/grub.cfg
    grub-mkrescue -o mykernel.iso isodir
    qemu-system-x86_64 -nographic -cdrom mykernel.iso
