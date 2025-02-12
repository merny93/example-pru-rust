# BeagleBone PRU mostly-Rust

This example show how the BeagleBone can be used for real-time tasks that are typically reserved for dedicated MCUs.
The BeagleBone is an ARM A8 processor with 2 dedicated co-processors called PRUs.
The board runs linux while the PRUs are "bare-metal"

This example does not have anything novel about it. 
It mostly just combines some of the examples from the [BeagleBone Cookbook](https://docs.beagleboard.org/beaglebone-cookbook.pdf) and the [PRU Cookbook](https://docs.beagleboard.org/books/pru-cookbook/index.html) along with the [prusst crate](https://github.com/sbarral/prusst).

## Building and Running

The entire project is designed to be cross-compiled for the BeagleBone from any linux machine.
On the rust side this requires the `armv7-unknown-linux-musleabihf` target.
As for the PRUs: TI provides a compiler that needs to be installed locally from [here](https://www.ti.com/tool/PRU-CGT).

The cargo cross-compilation is great - and my `.cargo/config.toml` will rsync all the binaries and run them on the BeagleBone.
The internet even claims that debugging across `ssh` is possible, but I have yet to try.