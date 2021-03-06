// hlcc-cli  -  hlcc command line interface
// Copyright (C) 2022 Jana Marie Hemsing

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    for input in args {
        let res = hlcc_parser::compute(&input);
        if let Some(out) = res {
            println!("{} computes to {}", input, out);
        } else {
            println!("{} does not compute :(", input);
        }
    }
}
