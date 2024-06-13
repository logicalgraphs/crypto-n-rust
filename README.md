# crypto-n-rust

We do crypto. We do Rust. We do crypto 'n Rust.

... Maaaayyyyybee we'll even add a little graph database work here 'n there.

## Pivots

Here we build a dAPP that recommends trades from @coingecko-data based
upon 
[pivot-arbitrage](https://logicalgraphs.blogspot.com/p/prism.html).

We have already built libraries around the book I'm writing (see below)
and around some 
[standard crypto-stuff I'm doing](https://github.com/logicalgraphs/crypto-n-rust/tree/main/src/libs/crypto).
Both are included into this git repository.

We introduce concepts iteratively via a series of 
[quizzes](src/pivot/quizzes).

## Crypto-n-Rust book

* Chapter 00: [Introduction](00-intro.md)
* Chapter 01: ["Hello, world!"](01-hello-world.md)
  * program: [greet.rs](src/ch01/greet.rs)
* Chapter 02: [Summer in the City](02-summer.md)
  * program: [summer.rs](src/ch02/summer.rs)
* Chapter 03: [Modules](03-modules.md) WIP
  * programs: [utils.rs](src/ch03/utils.rs) [winter.rs](src/ch03/winter.rs)

## Setup

Your `env` should contain:

* `RUST_BOOK` ... this directory
* `CARGO_HOME`=`$RUST_BOOK/src`

## Daily Reports

"How do I do the 'daily reports' that geophf does?" you ask.

Good question! [Here's](daily-reports.md) my answer.
