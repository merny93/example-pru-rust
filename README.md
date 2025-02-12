# BeagleBone PRU mostly-Rust

This example show how the BeagleBone can be used for real-time tasks that are typically reserved for dedicated MCUs.
The BeagleBone is an ARM A8 processor with 2 dedicated co-processors called PRUs.
The board runs linux while the PRUs are "bare-metal"

This example does not have anything novel about it. 
It mostly just combines some of the examples from the [BeagleBone Cookbook](https://docs.beagleboard.org/beaglebone-cookbook.pdf) and the [PRU Cookbook](https://docs.beagleboard.org/books/pru-cookbook/index.html) along with the [prusst crate](https://github.com/sbarral/prusst).

The code has two examples in it:

1. A simple multiply accumulate example that shows how memory is shared between the PRU and the main CPU and how the PRU can be used for a simple control loop. The PRU is a 200Mhz RISC processor with a 32 bit multiply accumulate accelerator making it comparable to simple MCUs
2. A Fibonacci sequence example that shows how the PRU can be used for a real-time task while the main CPU is doing other things. The PRU is not at the mercy of the linux scheduler so it has no jitter. This makes the PRU a good match for things like motor control or communication protocol decoding/encoding.

The "mostly-rust" is because the PRU code has to be written in C to be compiled by the `clpru` TI compiler. 
It is possible to write the PRU code in `no_std` rust and compile into C with the [`mrustc`](https://github.com/thepowersgang/mrustc) project and then feed it into `clpru` but this is probably not a good idea.
The PRU code should be fairly simple anyways and fundamentally unsafe so rust brings minimal benefit.

## Building and Running

The entire project is designed to be cross-compiled for the BeagleBone from any linux machine.
On the rust side this requires the `armv7-unknown-linux-musleabihf` target.
As for the PRUs: TI provides a compiler that needs to be installed locally from [here](https://www.ti.com/tool/PRU-CGT).

The cargo cross-compilation is great - and my `.cargo/config.toml` will rsync all the binaries and run them on the BeagleBone.
The internet even claims that debugging across `ssh` is possible, but I have yet to try.