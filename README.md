# hlcc :syringe:

hlcc is a Rust :crab: based hormone level conversion calculator. It expects a string with the desired hormone, its level and the target unit; e.g. "*Testo 1.8nmol/l in ng/ml*", "*e2 111pg/ml to nmol/l*" or "*Progesterone 0.07pg/ml to mol/l*", parses and computes the desired result.

A functioning webasm version is currently hostet at [hlcc.haj.gf](https://hlcc.haj.gf)

## using hlcc

The **hlcc library** itself can be found with the `hlcc-parser` folder. It exposes a `compute()` function, which takes a string slice (`&str`) and returns a string buffer (`String`). For more information see [hlcc-parser/README.md](https://github.com/Jana-Marie/hlcc/blob/main/hlcc-parser/README.md)

A **command line interface** for hlcc is available in the `hlcc-cli` directory, it can be compiled using `cargo build` or installed using `cargo install --path .`. It can compute either a single string or multiple strings at the same time; `hlcc_cli "t 1.8nmol/l to ng/dl"`; `hlcc_cli "t 1.8nmol/l to ng/dl" "e2 111ng/dl to nmol/l"`. For more information see [hlcc-cli/README.md](https://github.com/Jana-Marie/hlcc/blob/main/hlcc-cli/README.md)

It can als be compiled for **web as webassembly**, running `wasm-pack build` within the `hlcc-web` folder. The actual website can be packed running `webpack` while in the `www/` directory. You can of course use the instance I am hosting; [hlcc.haj.gf](https://hlcc.haj.gf). For more information see [hlcc-web/README.md](https://github.com/Jana-Marie/hlcc/blob/main/hlcc-web/README.md)

A **http api** service can be compiled from within the `hlcc-service` directory using `cargo-build`. It can be installed using `cargo install --path .`. The service expects its IP and Port as command line parameter; `hlcc-service "[::1]:8080"`. A request can be for example `https://api.hlcc.haj.gf/?q=t%201.8nmol/l%20to%20ng/dl`, you can again use the service running at [api.hlcc.haj.gf](https://api.hlcc.haj.gf/?q=). For more information see [hlcc-service/README.md](https://github.com/Jana-Marie/hlcc/blob/main/hlcc-service/README.md)

## features

It currently features all SI prefixes, a bunch of major hormones like *Cortisol*, *Cholesterol*, *E1*, *E2*, *E3*,*E4*, *T*, *DHT* and *P4*, other blood level values, as well as *Gram* ⇋ *Mole* conversions. In the future it should also be able to convert *Gram* ⇋ *Mole* ⇋ *Volume*, and there will be more units.

## parser in depth

Okay, here is what the parser parses, or how you are supposed to build your input. It must be in the following format:

`<hormone> <number><(O)prefix><unit>/<(O)prefix><unit> <conjunction> <(O)prefix><unit>/<(O)prefix><unit>`

All prefixes are optional, a unit can have no prefix. All whitespaces can be of lenght 1 or longer, there can be optional whitespaces inbetween the number and the prefix. The conjunction can either be "in", "to", ">" or "->". As for the Hormone, either the full name, a common short version or the first letter (+number) can be used. In case of Estradiol, "Estradiol" or "E2". All units must be metric and can either be Gram "g", Litre "l" or Mole "mol". Prefixes must be SI prefixes.

There was this thought while writing this, that this tool could convert almost all blood level values to different units. I should build another version with much more stuff.

## about the project

After converting my own hormone levels once again by hand, and after using plenty online converter with simply unbearable user interfaces, I decided to write my own conversion tool. Since I really wanted to learn Rust (insert trans catgirl Rust meme here), I had to take this opportunity. I love the idea of „asking“ a computer a question, or rather forming your problem into a sentence and get it answered. Thus I decided to write a parser, which can take such a string, understand and compute it. This probably is not the easiest project to start with, but I had plenty of help by wonderful people and in the end, it worked out really well. Rust can be learned, even by a hardware catgirl like me. :3

Oh and about the web interface, it is shitty, I know, I sincerely don't get CSS, nor JS \*hisses at web stuff\*.

## todo

 - [ ] Rust: change f64 to d128 to avoid floating point rounding errors (decimal = "2.1")
 - [x] Rust: add pretty print for output unit
 - [x] Rust: get command line tool to work
 - [x] Rust: add hormones: aldosterone, Gonadotropin, Gonadorhelin, SHBG,
 - [ ] Rust: remove panic, add proper error reporting
 - [x] Rust: add other blood and vital levels
 - [x] Rust: Bug: "t 1.8nmol/l to ng/d" produces a numeric result, it shouldn't
 - [x] Rust & JS: &str instead of String
 - [x] Docs: add license
 - [ ] Docs: work on parser library readme
 - [x] Web: change input fielt to form
 - [ ] Web: move output value to righthandside of the Input
 - [x] Web: have a prettier input field
 - [x] Web: add better scaling
 - [x] Web: h1 font pink
 - [x] Web: fix JS, should be different handler (form handler)
