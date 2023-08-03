// hlcc-cli  -  hlcc command line interface
// Copyright (C) 2022 Jana Marie Hemsing

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    for input in args {
        match hlcc_parser::compute(&input) {
            Ok(out) => println!("{} computes to {}", input, out),
            Err(e) => println!("{} :(", e),
        }
    }
}
