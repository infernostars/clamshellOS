cargo build --package clamshell --bin clamshell
qemu-img convert -f raw -O vdi target/x86_64-clamshell/debug/bootimage-clamshell.bin build/image.vdi

