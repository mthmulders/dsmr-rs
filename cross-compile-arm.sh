#!/bin/bash
set -euo pipefail

rustup target add arm-unknown-linux-musleabihf

ls -hl $ARMHF_TOOLCHAIN_PATH/bin

export CC_arm_unknown_linux_gnu=arm-none-linux-gnueabihf-gcc
export CXX_arm_unknown_linux_gnu=arm-none-linux-gnueabihf-g++
export AR_arm_unknown_linux_gnu=arm-none-linux-gnueabihf-ar
export CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSCLEABIHF_LINKER=arm-none-linux-gnueabihf-gcc

export PATH=$ARMHF_TOOLCHAIN_PATH/bin:$PATH
export PKG_CONFIG=/usr/bin/arm-unknown-linux-musleabihf-pkg-config

cargo build --target arm-unknown-linux-musleabihf --release --features ""
