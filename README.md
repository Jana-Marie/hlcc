# hlcc :syringe:

hlcc is a Rust :crab: and web based hormone level conversion calculator. It expects a string with the desired hormone, its level and the target unit; e.g. "*Testo 1.8nmol/l in ng/ml*", "*e2 111pg/ml to nmol/l*", "*Progesterone 0.07pg/ml to mol/l*". And computes the desired result. A functioning version is currently hostet at [hlcc.haj.gf](https://hlcc.haj.gf)

## building hlcc

It can be compiled for web, running `wasm-pack` within the reposity and running `webpack` within the www/ directory.

There is also an option to compile hlcc to a command line tool, using `rustc`.

## features

It currently features all SI prefixes, the hormones E2, T and P4, as well as Gram <-> Mole conversions. In the future it should also be able to convert Gram <-> Mole <-> Volume, and there will be more hormones.

## about the project

After converting my own hormone levels once again by hand, and after using plenty only converter with simply unbearable user interfaces, I decided to write my own conversion tool. Since I really wanted to learn Rust (insert trans catgirl Rust meme here), I had to take this opportunity. I love the idea of „asking“ a computer a question, or rather forming your problem into a sentence and get it answered. Thus I decided to write a parser, which can take such a string, understand and compute it. This probably is not the easiest project to start with, but I had plenty of help by wonderful people and in the end, it worked out really well. Rust can be learned, even by a hardware catgirl like me. :3
Oh and about the web interface, it is shitty, I know, I sincerely don't get CSS, nor JS \*hisses at web stuff\*.
