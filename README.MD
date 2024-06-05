# Nyaos
A project revolving around building a operating system from scratch. Built entirely within rust!

## Build from source:
Requirements:
- Rust (nightly)
- Bootimage (run `cargo install bootimage`)

To build, simply run `cargo bootimage` and grab your bin file from the target directory <br>
Or to start qemu with the generated binary `cargo run` will work. <br>
To run with no display from qemu, run `cargo run -- --serial stdio --display none` (Only use this in cases when needed, some features simply dont work when running in serial mode)