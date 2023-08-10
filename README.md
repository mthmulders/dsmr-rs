# dsmr-rs

![CI build](https://github.com/mthmulders/dsmr-rs/workflows/CI%20build/badge.svg)

A utility tool to ship data from a smart energy meter over HTTP.

## Running
Download the Debian packages from the [release area](https://github.com/mthmulders/dsmr-rs/releases).
Copy it to the machine where you want to run the process and install it with `dpkg -i  dsmr-rs_<version>_<arch>.deb`.
Edit `/etc/dsmr-rs.conf` to tailor your configuration.
Finally, run `sudo service dsmr-rs restart` to make your changes effective.

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
