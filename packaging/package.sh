#!/bin/bash
VERSION=0.1.0
set -e 
rm *.deb
cd "$(dirname "$0")"
mkdir workspace
mkdir -p workspace/usr/bin
mkdir -p workspace/etc/systemd/system
cd ..
HOST=x86_64-unknown-linux-gnu cargo build --target=armv7-unknown-linux-gnueabihf --release
cp target/armv7-unknown-linux-gnueabihf/release/rustysnake packaging/workspace/usr/bin
cd -
cp rustysnake.service workspace/etc/systemd/system
fpm -C workspace -n rustysnake -v $VERSION --description "Rust-based version of Snake to run on raspberry pi with unicorn hat hd" -a armhf -s dir -t deb --license MIT \
	-m "Paul Praet" --url https://github.com/praetp/rustysnake --after-install afterinstall --iteration $(date +%s)
rm -rf workspace
