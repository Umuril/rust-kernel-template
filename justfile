set dotenv-load

TARGET_DIR := if env("PROFILE") == "dev" {
  "debug"
} else {
  env("PROFILE")
}

default:
    @just --list --justfile {{justfile()}}

build:
    cargo clean # Force rebuild to get changes to linker scripts. Move to build.rs in the future
    cargo rustc --profile=$PROFILE --target=targets/$ARCH.json -- -Clink-arg=-T -Clink-arg=linkers/$ARCH.ld -Clink-arg=-z -Clink-arg=nognustack

readelf: build
    $BINUTILS-readelf -a target/$ARCH/{{TARGET_DIR}}/rust-kernel-template -C rust -W

qemu gdb="false" nographic="false": build
    rm -fr ./isodir
    mkdir -p isodir/boot/grub
    cp target/$ARCH/{{TARGET_DIR}}/rust-kernel-template isodir/boot/kernel.bin
    echo 'set timeout=0\nset default=0\n\nmenuentry "My Kernel" {\nmultiboot /boot/kernel.bin\nboot\n}' > isodir/boot/grub/grub.cfg
    grub-mkrescue -o mykernel.iso isodir
    qemu-system-x86_64 {{ if gdb == "true" { "-s -S" } else { "" } }} -serial stdio -cdrom mykernel.iso {{ if nographic == "true" { "-nographic" } else { "" } }}

gdb:
    @until [ -e target/$ARCH/{{TARGET_DIR}}/rust-kernel-template ]; do sleep 1; done
    gdb-multiarch -ex 'target remote localhost:1234' target/$ARCH/{{TARGET_DIR}}/rust-kernel-template

clean:
    cargo clean
    rm -fr ./mykernel.iso ./isodir