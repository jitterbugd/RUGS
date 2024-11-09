# RUGS (RUdementary Graphic Storage)

![License](https://www.gnu.org/graphics/gplv3-or-later.png)

## Description

An image format engineered to be simple and compact, inspired by greatly the popular format PNG.

## Features

- Smaller file size than PNG (in most cases)
- Various lossy compression support
- Baked-in ZLib compression
- Engineered with simplicity in mind

## Header Structure
Each segment of the header is exactly four bytes long, and all integers are 32-bit and formatted using Big Endian.

1. Magic Bytes: A unique identifier that is essentially "RUGS" in ASCII.
2. Height: The height of the image as an integer.
3. Width: The width of the image as an integer.
4. Is Compressed: An integer which is either a one if the image has been lossily compressed before or a zero if it hasn't.

## Compilation (Powershell)
This will compile and run the commandline utility for RUGS.
You must have [Rust](https://www.rust-lang.org/learn/get-started) installed to compile it.  
   ```bash
   git clone https://github.com/jitterbugd/RUGS; cd RUGS; cargo run --release
 ```
## Installation (Windows)
Visit the releases page [here](https://github.com/jitterbugd/RUGS/releases) and download the binary of your choice. It is a commandline program, so you must execute it from one.
