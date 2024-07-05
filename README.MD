# Nyaos
A project revolving around building an operating system from scratch. Built entirely with Rust!

## Build from Source

There are some ways to build it from source, but most of it boils down to running the Makefile.

### Dependencies (Compile without Docker)

- Rust (nightly compiler > 1.80)
- Make
- NASM
- GRUB `grub-mkrescue`
- xorriso

### Dependencies (Compile with Docker)

- Docker

### Build Steps

#### Windows

Run `start-vm.bat`

#### Linux & macOS

Run `start-vm.sh`

### Build

Run the `./make.sh` file to automatically run all the build steps. (Linux only)
