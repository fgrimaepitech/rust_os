# Rust OS

![CleanShot 2025-05-18 at 18 18 50@2x](https://github.com/user-attachments/assets/8a3da09f-6f08-4a84-a660-5ed96b5bf06f)

## Build

`cargo bootimage`

## Run with QEMU

`qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin`
