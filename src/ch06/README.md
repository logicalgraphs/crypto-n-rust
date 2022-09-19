# cmc_filter

Reads the specified CSV file of price-quotes AND reads the file containing
held assets, THEN generates a report (CSV) of held asset prices.

### Usage:

```SHELL
./cmc_filter <prices CSV file> <held assets LSV file>

	prints CSV of prices of held assets only.
```

### e.g.:

```SHELL
$ cargo run $RUST_BOOK/data-files/csv/listings-2022-09-18.csv ../portfolio_coins.lsv

prices has 5003 lines
assets has 4 lines
Could not process line 2022-09-18,4953,481,FCT,FCT2,FirmaChain,$0.06
date,cmc_id,rank,symbol,name,price
2022-09-18,1975,23,LINK,Chainlink,$7.96
2022-09-18,3945,117,ONE,Harmony,$0.02
```

### see

e.g.: `/data-files/csv/portfolio_coins.lsv` for a list of held assets.

## Compilation

Like the [previous project](../ch05/README.md), this is a Cargo project, so
you follow the procedure as before, creating a new project, copying the Cargo
file into the project directory, then linking in the assets (the Rust files
in this directory and then the library files from `lib.rs` in prior 
directories).

The. End.
