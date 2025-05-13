set dotenv-load
set shell := ["bash", "-cu"]

TARGET_DIR := if env("PROFILE") == "dev" {
  "debug"
} else {
  env("PROFILE")
}

default:
    @just --list --justfile {{justfile()}}

build:
    cargo clean # Force rebuild to get changes to linker scripts.
    cargo fmt
    cargo rustc --profile=$PROFILE --target=targets/$ARCH.json -- -Clink-arg=-T -Clink-arg=linkers/$ARCH.ld -Clink-arg=-z -Clink-arg=nognustack
    cargo clippy --profile=$PROFILE --target=targets/$ARCH.json

readelf: build
    $BINUTILS-readelf -C rust -W -a target/$ARCH/{{TARGET_DIR}}/rust-kernel-template 

qemu gdb="false" nographic="false": build
    @if [[ $ARCH == "x86_64" ]]; then just qemu-x86_64 {{gdb}} {{nographic}}; fi
    @if [[ $ARCH == "arm" ]]; then just qemu-arm {{gdb}} {{nographic}}; fi

qemu-x86_64 gdb="false" nographic="false":
    rm -fr ./target/isodir
    mkdir -p target/isodir/boot/grub
    cp target/$ARCH/{{TARGET_DIR}}/rust-kernel-template target/isodir/boot/kernel.bin
    echo -e 'set timeout=0\nset default=0\n\nmenuentry "My Kernel" {\nmultiboot /boot/kernel.bin\nboot\n}' > target/isodir/boot/grub/grub.cfg
    grub-mkrescue -o target/mykernel.iso target/isodir
    qemu-system-x86_64 {{ if gdb == "true" { "-s -S" } else { "" } }} -serial stdio -cdrom target/mykernel.iso {{ if nographic == "true" { "-nographic" } else { "" } }}

qemu-arm gdb="false" nographic="false":
    qemu-system-arm -M virt -cpu cortex-a15 {{ if gdb == "true" { "-s -S" } else { "" } }} -kernel target/$ARCH/{{TARGET_DIR}}/rust-kernel-template -serial stdio {{ if nographic == "true" { "-nographic" } else { "" } }}

gdb:
    @until [ -e target/$ARCH/{{TARGET_DIR}}/rust-kernel-template ]; do sleep 1; done
    gdb-multiarch -ex 'target remote localhost:1234' target/$ARCH/{{TARGET_DIR}}/rust-kernel-template

clean:
    cargo clean
    