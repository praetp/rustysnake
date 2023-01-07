#!/bin/bash
set -ex
REMOTE_HOST=shumeipai.local
#TARGET=armv7-unknown-linux-gnueabihf #32-bit
#TARGET=aarch64-unknown-linux-gnu #64-bit
TARGET=armv7-unknown-linux-musleabihf
cross build --verbose --target=${TARGET}
rsync -av target/${TARGET}/debug/rustysnake ${REMOTE_HOST}: 
RUST_BACKTRACE=1 ssh -t ${REMOTE_HOST} /home/spetsnaz/rustysnake /dev/input/event1
echo "done"
