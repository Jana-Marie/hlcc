# hlcc :syringe:

hlcc is a Rust :crab: and web based hormone level conversion calculator. It expects a string with the desired hormone, its level and the target unit; e.g. "*Testo 1.8nmol/l in ng/ml*", "*e2 111pg/ml to nmol/l*" or "*Progesterone 0.07pg/ml to mol/l*". And computes the desired result.

A functioning version is currently hostet at [hlcc.haj.gf](https://hlcc.haj.gf)

## building hlcc

It can be compiled for web, running `wasm-pack` within the reposity. The actual web stuff can be packed running `webpack` while in the www/ directory.

There is also an option to compile hlcc to a command line tool, using `cargo build`. <- this does not work yet

## features

It currently features all SI prefixes, the hormones *E2*, *T* and *P4*, as well as *Gram* <—> *Mole* conversions. In the future it should also be able to convert *Gram* <—> *Mole* <—> *Volume*, and there will be more hormones.

## parser in depth

Okay, here is what the parser parses, or how you are supposed to build your input. It must be in the following format:

`<hormone> <number><(O)prefix><unit>/<(O)prefix><unit> <conjunction> <(O)prefix><unit>/<(O)prefix><unit>`

All prefixes are optional, a unit can have no prefix. All whitespaces can be of lenght 1 or longer, there can be optional whitespaces inbetween the number and the prefix. The conjunction can either be "in", "to", ">" or "->". As for the Hormone, either the full name, a common short version or the first letter (+number) can be used. In case of Estradiol, "Estradiol" or "E2". All units must be metric and can either be Gram "g", Litre "l" or Mole "mol". Prefixes must be SI prefixes.

There was this thought while writing this, that this tool could convert almost all blood level values to different units. I should build another version with much more stuff.

## about the project

After converting my own hormone levels once again by hand, and after using plenty only converter with simply unbearable user interfaces, I decided to write my own conversion tool. Since I really wanted to learn Rust (insert trans catgirl Rust meme here), I had to take this opportunity. I love the idea of „asking“ a computer a question, or rather forming your problem into a sentence and get it answered. Thus I decided to write a parser, which can take such a string, understand and compute it. This probably is not the easiest project to start with, but I had plenty of help by wonderful people and in the end, it worked out really well. Rust can be learned, even by a hardware catgirl like me. :3

Oh and about the web interface, it is shitty, I know, I sincerely don't get CSS, nor JS \*hisses at web stuff\*.
