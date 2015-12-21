# eVic
[![Build Status](https://travis-ci.org/mkroman/evic.svg)](https://travis-ci.org/mkroman/evic)

Command-line utility for encrypting and decrypting firmware for the eVic-VTC
Mini.

## Manual installation

To install `evic` and `evicutil`, you need to have the latest Rust stable
installed, for more information on Rust, see https://rust-lang.org/

If you have Rust installed, all you have to do is run

`cargo install evic`

And then make sure your cargo bin path (typically `~/.cargo/bin` is in your
PATH.)

## Usage

### Encrypting firmware

To encrypt a piece of firmware, run `evicutil` with the `encrypt` command.

`evicutil encrypt firmware.bin`

This will encrypt the firmware.bin file and save it as `firmware_encrypted.bin`.

### Decrypting firmware

To decrypt a piece of firmware, run `evicutil` with the `decrypt` command.

`evicutil decrypt firmware.bin`

Optionally, you can run the command with an `-o` option followed by an output
path, for example:

`evicutil decrypt -o output.bin firmware.bin`

which will decrypt `firmware.bin` and save it as `output.bin`.

# License

This work is published under the MIT license.
