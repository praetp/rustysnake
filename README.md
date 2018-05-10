# Introduction
Rustysnake is a Rust-based implementation of the classic _snake_ game which you may have played on your Nokia phone in the past.

What makes it special ?
* It's implemented in Rust.
* It's designed to run on a Raspberry PI with a Unicorn Hat HD.
* It's designed to take it's input from a USB input device that supports buttons typically found on a gamepad.
  FYI: I am using [this one](https://www.onlinekabelshop.nl/snes-style-usb-controller-voor-pc-notebook-1-35-meter).

# Why ?
I was interested to learn more about Rust and the only way to learn a new programming language is by _using_ it. 
To make it a little more interesting than just a console application, I decided to make a small game for my daughter on the Raspberry Pi. 

# How to build
If you want to build for your host PC, first modify one line in Cargo.toml and then
```
cargo build
```

If you want to cross-compile for the raspberry pi (assuming you have the toolchain installed).

```
HOST=x86_64-unknown-linux-gnu cargo build --target=armv7-unknown-linux-gnueabihf
```

# How to run
The program only takes one argument: where the device file of the input device can be found.
Typically, you can find it as `/dev/input/event0` if your raspberry pi only has one input device.

# How to play
Use the arrow to move the snake around. Whenever the snake eats the bait, the snake grows one unit. 
Also, the game goes a little faster after every successful bite. 
If the snake "eats" itself, the game ends.
Pressing the color buttons on the controller, changes the color of the snake (you can thank my daughter for this feature).
