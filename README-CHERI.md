# Rust for CHERI targets

This README describes how to build and use Rust for [CHERI] targets, like Arm's
Morello platform.

[CHERI]: https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/

### Status

### Targets

* `riscv32imacxcheri-unknown-none-purecap`

A RISC-V target, intended as a test bed for potential embedded CHERI
processors. Not tested.

* `morello-unknown-none-purecap`

A freestanding Morello target, a midpoint as a compiler target to help get us
to the full 'Morello with OS' target. Not tested.

* `morello-unknown-freebsd-purecap`

A Morello target with CheriBSD, currently the best supported real-hardware
CHERI target. Currently WIP, basic compilation should work.

### Building and Running

This README currently covers building for the `morello-unknown-freebsd-purecap`
target.

1. Building the Morello CheriBSD SDK

Rust for Morello CheriBSD requires the use of the CheriBSD Morello SDK. This
can be installed using [cheribuild], by running
`./cheribuild.py cheribsd-sdk-morello-purecap`. Add the build binaries in
`/home/<user>/cheri/output/morello-sdk/bin` to your `PATH`.

[cheribuild]: https://github.com/CTSRD-CHERI/cheribuild

2. Building Rust for Morello CheriBSD

To build the Rust compiler itself, this repository should be cloned using git,
followed by running the command `git submodule update --recursive`. Then, copy
the file `config.toml.morello` to `config.toml`, and start the build with the
command `./x.py build --target=morello-unknown-freebsd-purecap library/std`.

3. Running the compiler

Add the directory `./build/<host>/stage1/bin` to the `PATH`. To compile an
executable with the resulting compiler, run the command:

```
rustc --target=morello-unknown-freebsd-purecap \
-C linker=aarch64-unknown-freebsd-purecap \
-C link-args=-fuse-ld=lld \
-C link-args=--sysroot=/home/<user>/cheri/output/rootfs-morello-purecap
```
