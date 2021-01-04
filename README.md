# dsmr-rs

![CI build](https://github.com/mthmulders/dsmr-rs/workflows/CI%20build/badge.svg)

A utility tool to ship data from a smart energy meter over HTTP.

## Development

### Testing
Run tests with `cargo t`.

### Building
Build a debug binary with `cargo b`.
For a release binary, add `--release`.

If you want to run on a Raspberry Pi, you need to download the GNU Toolchain from ARM.
[Cross Compiling Rust for the Raspberry Pi](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/) explains the details for that.
After that, add `--target armv7-unknown-linux-gnueabihf` to the above command. 