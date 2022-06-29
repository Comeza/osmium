A simple x86_64 kernel to test stuff out

## Requirements
 - [QEMU](https://www.qemu.org/)
 - [RUST](https://www.rust-lang.org/)

## Building
```sh
cargo build
```
You will find an os.efi in the target directory that you can copy to a fat32
formatted USB under /boot/efi/bootx64.efi

## Run
```sh
cargo run
```
Should start an QEMU instance with the kernel.

