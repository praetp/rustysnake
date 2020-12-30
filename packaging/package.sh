#!/bin/bash
# this script requires fpm (see https://fpm.readthedocs.io/en/latest/installing.html)
VERSION=0.1.2
rm *.deb
set -e 
cd "$(dirname "$0")"
mkdir workspace
mkdir -p workspace/usr/bin
mkdir -p workspace/etc/systemd/system
cd ..
cargo build --target=armv7-unknown-linux-gnueabihf --release
cp target/armv7-unknown-linux-gnueabihf/release/rustysnake packaging/workspace/usr/bin
cd -
cp rustysnake.service workspace/etc/systemd/system
fpm -C workspace -n rustysnake -v $VERSION --description "Rust-based version of Snake to run on raspberry pi with unicorn hat hd" -a armhf -s dir -t deb --license MIT \
	-m "Paul Praet" --url https://github.com/praetp/rustysnake --after-install afterinstall --iteration $(date +%s)
rm -rf workspace
