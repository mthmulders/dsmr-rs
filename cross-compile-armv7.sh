#!/bin/bash
set -euo pipefail

PATH=$ARMHF_TOOLCHAIN_PATH/bin:$PATH \
	cargo build --target armv7-unknown-linux-gnueabihf --release --features ""
