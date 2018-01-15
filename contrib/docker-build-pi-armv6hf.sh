#!/usr/bin/env bash
set -eux

SYSROOT_DIR="/pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot"

# Add extra target dependencies to our rpi sysroot
DEPS=( \
http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2_1.0.25-4_armhf.deb \
http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2-dev_1.0.25-4_armhf.deb \
)
for path in "${DEPS[@]}"; do
    curl -OL $path
    ar p $(basename $path) data.tar.gz | tar -xz -C $SYSROOT_DIR
done
# i don't why this is neccessary
#ln -s ld-linux.so.3 $SYSROOT_DIR/lib/ld-linux-armhf.so.3

GCC_BIN_DIR="/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin"

# Some crates want to work this out for themselves and otherwise get it wrong 
export TARGET_CC="$GCC_BIN_DIR/arm-linux-gnueabihf-gcc"

# create wrapper around gcc to point to rpi sysroot
GCC_WRAPPER="$GCC_BIN_DIR/gcc-sysroot"
echo -e '#!/bin/bash' "\n$TARGET_CC --sysroot $SYSROOT_DIR \"\$@\"" > $GCC_WRAPPER
chmod +x $GCC_WRAPPER

# point cargo to use our gcc wrapper
echo -e "[target.arm-unknown-linux-gnueabihf]\nlinker = \"$GCC_WRAPPER\"" > /.cargo/config

# Build!
cd /src
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend

