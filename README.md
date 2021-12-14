# rust-nds-template
Project template to develop DS hombrew using Rust

# Setup
Make sure you have installed the devkitPro toolchain [Getting Started], just Rust is not enough.

Copy this template to its own place, either using the "Use this template" button on GitHub or downloading it as Zip.
(Just make sure the Git repo doesn't point to this repo)

After that just update the [Cargo.toml](./Cargo.toml) and start editing the template!

# Usage
The [Makefile](./Makefile) contains the commands (and wrappers) you need to build the project:
 - `make build`: Calls `cargo build` and creates the `.nds`
 - `make clean`: Calls `cargo clean` and deletes all `.nds`
 - `make update`: Calls `cargo update` with some configuration
 - `make run`: Calls `make build` and runs the ROM in [melonDS] (configurable, change `runner` in [Makefile](./Makefile))
By default a release version is built. Use `DEBUG=1` (e. g.: `make DEBUG=1 run`) to build debug versions

You shouldn't call Cargo directly, since it will only build the `.elf` (the code of your ROM)

[Getting Started]: https://devkitpro.org/wiki/Getting_Started
[melonDS]: http://melonds.kuribo64.net
