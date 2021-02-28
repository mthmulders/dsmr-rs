#!/bin/bash
set -euo pipefail

# echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal main" > /etc/apt/sources.list.d/ubuntu-ports.list
# echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal-security main" > /etc/apt/sources.list.d/ubuntu-ports.list
# echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates main" > /etc/apt/sources.list.d/ubuntu-ports.list
# sudo aptitude update
# sudo aptitude install pkg-config-arm-linux-gnueabihf libssl-dev:armhf libudev-dev:armhf

rustup target add armv7-unknown-linux-gnueabihf

ls -hl $ARMHF_TOOLCHAIN_PATH/bin

PATH=$ARMHF_TOOLCHAIN_PATH/bin:$PATH \
	PKG_CONFIG=/usr/bin/arm-linux-gnueabihf-pkg-config \
	cargo build --target armv7-unknown-linux-gnueabihf --release --features ""
