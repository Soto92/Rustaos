# Rustaos Portable

Rustaos Portable is a lightweight, custom-built operating system written entirely in Rust, designed to run directly from a USB flash drive. Its primary focus is portability, security, and low-level control, offering a minimal yet expandable environment for learning, experimentation, or specialized tasks.

This project serves both as a personal OS experiment and as an educational platform to explore systems programming, memory management, hardware interfacing, and boot processes — all through the modern and memory-safe Rust language.

## Features

- Cross-compilation with Rust
- Bootloader compatible with the target architecture
- Specific initialization code (assembly and Rust)
- QEMU or compatible real hardware for testing

## Install Rust and Cargo

[Rust](https://www.rust-lang.org/learn/get-started)

### Add support to Cross-compilation (installation, choose one or install all):

- x86_64

```
rustup target add x86_64-unknown-none
```

- RISC-V 64:

```
rustup target add riscv64gc-unknown-none-elf
```

- RM Cortex-A:

```
rustup target add aarch64-unknown-none
```

## 🛠️ 1. **x86_64 (eg: BIOS boot via VGA)**

### ⚙️ Compile:

```bash
cargo build --release --target x86_64-unknown-none
```

### 🔗 Link:

Merge bootloader + kernel:

```bash
cat bootloader/boot.bin target/x86_64-unknown-none/release/rustaos > os-image-x86_64.bin
```

- Windows:

```
Get-Content -Raw bootloader\boot.bin, target\x86_64-unknown-none\release\rustaos | Set-Content -Encoding Byte os-image-x86_64.bin
```

---

## 🛠️ 2. **RISC-V (riscv64gc-unknown-none-elf)**

### ⚙️ Compile:

```bash
cargo build --release --target riscv64gc-unknown-none-elf
```

### 🔗 Link (QEMU):

```bash
qemu-system-riscv64 -machine virt -nographic -bios none -kernel target/riscv64gc-unknown-none-elf/release/rustaos
```

## 🛠️ 3. **ARM 64-bit (aarch64 / Raspberry Pi 3/4)**

### ⚙️ Compile:

```bash
cargo build --release --target aarch64-unknown-none
```

### 🔗 Link:

```bash
rust-objcopy target/aarch64-unknown-none/release/rustaos -O binary rustaos-aarch64.bin
```

Run:

```bash
qemu-system-aarch64 -M raspi3 -kernel rustaos-aarch64.bin -serial null -serial stdio
```

### Obs.:

Bootloader is required only for x86_64

## Clean cache

```
cargo clean
```

### Testing with QEMU

```
qemu-system-riscv64 -machine virt -nographic -bios none -kernel target/riscv64gc-unknown-none-elf/debug/rustaos
```

## Record it to a USB flash drive (MacOS or Linux)

```
sudo dd if=os-image.bin of=/dev/sdX bs=512 seek=0
```

### For windows you can use Rufus

[Rufus](https://rufus.ie/en/)

## References and links

[Tutorial by Philipp Oppermann](https://os.phil-opp.com)

[SO in Zig by Diego Pacheco](https://github.com/diegopacheco/zos)
