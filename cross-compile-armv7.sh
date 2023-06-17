#!/bin/bash
set -euo pipefail

rustup target add armv7-unknown-linux-gnueabihf

ls -hl $ARMHF_TOOLCHAIN_PATH/bin

PATH=$ARMHF_TOOLCHAIN_PATH/bin:$PATH \
	PKG_CONFIG=/usr/bin/arm-linux-gnueabihf-pkg-config \
	cargo build --target armv7-unknown-linux-gnueabihf --release --features ""
