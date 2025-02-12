use std::env;
use std::path::Path;
use std::process::Command;

//run with -vv to see output of build script

fn main() {
    // Change to the PRU directory
    let pru_dir = Path::new("pru");
    // Tell cargo to rerun if any files in pru/ change
    println!("cargo:rerun-if-changed=pru/");

    // Run make clean and make
    assert!(Command::new("make")
        .arg("clean")
        .current_dir(pru_dir)
        .status()
        .expect("Failed to run make clean")
        .success());

    assert!(Command::new("make")
        .current_dir(pru_dir)
        .status()
        .expect("Failed to build PRU firmware")
        .success());
}
