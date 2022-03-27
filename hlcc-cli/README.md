# hlcc-cli

Hlcc-cli is a command line parser for hormone level strings. It returns the computed result, such a string might be „Testosterone 1.8nmol/l to ng/dl“.

It can be compiled using `cargo build` and installed using `cargo install --path .`.

It can compute either a single string or multiple strings at the same time; `hlcc_cli "t 1.8nmol/l to ng/dl"`; `hlcc_cli "t 1.8nmol/l to ng/dl" "e2 111ng/dl to nmol/l"`. 
