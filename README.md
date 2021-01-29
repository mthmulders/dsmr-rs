# dsmr-rs

![CI build](https://github.com/mthmulders/dsmr-rs/workflows/CI%20build/badge.svg)

A utility tool to ship data from a smart energy meter over HTTP.

## Running
Copy the sample [SupervisorD configuration](./sample-supervisord-config) to **/etc/supervisor/conf.d**.
Issue `sudo supervisorctl reload` followed by `sudo supervisorctl start dsmr_native_logger`.

## Development

### Testing
Run tests with `cargo t`.

### Building
Build a debug binary with `cargo b`.
For a release binary, add `--release`.

If you want to run on a Raspberry Pi, you need to download the GNU Toolchain from ARM.
[Cross Compiling Rust for the Raspberry Pi](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/) explains the details for that.
After that, add `--target armv7-unknown-linux-gnueabihf` to the above command. 