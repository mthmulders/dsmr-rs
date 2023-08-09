#!/bin/bash
set -euo pipefail

rustup target add arm-unknown-linux-musleabihf

ls -hl $ARMHF_TOOLCHAIN_PATH/bin

export CC_arm_unknown_linux_gnu=arm-unknown-linux-musleabihfgcc
export CXX_arm_unknown_linux_gnu=arm-unknown-linux-musleabihf-g++
export AR_arm_unknown_linux_gnu=arm-unknown-linux-musleabihf-ar
export CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSCLEABIHF_LINKER=arm-unknown-linux-musleabihf-gcc

export PATH=$ARMHF_TOOLCHAIN_PATH/bin:$PATH
export PKG_CONFIG=/usr/bin/arm-unknown-linux-musleabihf-pkg-config

cargo build --target arm-unknown-linux-musleabihf --release --features ""

# PATH=/opt/homebrew/Cellar/armv7-unknown-linux-musleabihf/11.2.0_1/toolchain/bin/:$PATH \
# 	cargo build --target arm-unknown-linux-musleabihf --release --features ""

# Suppose you have installed x86_64-unknown-linux-gnu toolchain and have it on PATH,
# setup the environment variables as below to use it with Cargo.
# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

# installed armv7-unknown-linux-musleabihf

# export CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSCLEABIHF_LINKER=arm-unknown-linux-musleabihf
# /bin/sh: arm-linux-musleabihf-gcc: command not found
# arm-unknown-linux-musleabihf
