# dsmr-rs

![CI build](https://github.com/mthmulders/dsmr-rs/workflows/CI%20build/badge.svg)

A utility tool to ship data from a smart energy meter over HTTP.

## Running
Copy the sample [SupervisorD configuration](./sample-supervisord-config) to **/etc/supervisor/conf.d**.
Issue `sudo supervisorctl reload` followed by `sudo supervisorctl start dsmr_native_logger`.

## Efficiency
The standard datalogger that ships with DSMR reader is written in Python.

Using `sudo pmap <pid> | tail -n 1`, its memory usage is reported as 37120K.
This native logger uses only 14996K, a reduction of almost 60%.

Using `ps aux | grep python3`, CPU usage of the Python logger is reported between 1.5% and 3% on my Raspberry Pi 4.
This native logger usually takes less than 1% of CPU time.

## Development

### Testing
Run tests with `cargo t`.

### Building
Build a debug binary with `cargo b`.
For a release binary, add `--release`.

If you want to run on a Raspberry Pi, you need to download the GNU Toolchain from ARM.
[Cross Compiling Rust for the Raspberry Pi](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/) explains the details for that.
After that, add `--target armv7-unknown-linux-gnueabihf` to the above command. 